//! Manta Simulation

use clap::{App, Arg};
use core::cmp::min;
use core::ops::Range;
use indexmap::IndexMap;
use rand::{distributions::Distribution, seq::SliceRandom, thread_rng, Rng, RngCore};
use serde::{Deserialize, Serialize};
use statrs::{
    distribution::{Categorical, Discrete, Poisson},
    StatsError,
};
use std::collections::HashMap;
use std::fs;

/// Flushes the STDOUT buffer.
#[inline]
fn flush_stdout() {
    use std::io::Write;
    let _ = std::io::stdout().flush();
}

/// Choose `count`-many elements from `vec` randomly and drop the remaining ones.
#[inline]
fn choose_multiple<T, R>(vec: &mut Vec<T>, count: usize, rng: &mut R)
where
    R: RngCore + ?Sized,
{
    let drop_count = vec.partial_shuffle(rng, count).1.len();
    vec.drain(0..drop_count);
}

/// Asset Id
pub type AssetId = u32;

/// Asset Value
pub type AssetValue = u128;

/// Asset
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    /// Asset Id
    pub id: AssetId,

    /// Asset Value
    pub value: AssetValue,
}

impl Asset {
    /// Builds a new [`Asset`] from the given `id` and `value`.
    #[inline]
    pub fn new(id: AssetId, value: AssetValue) -> Self {
        Self { id, value }
    }
}

/// Balance State
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, transparent)]
pub struct BalanceState {
    /// Asset Map
    map: HashMap<AssetId, AssetValue>,
}

impl BalanceState {
    /// Returns the asset balance associated to the assets with the given `id`.
    #[inline]
    pub fn balance(&self, id: AssetId) -> AssetValue {
        self.map.get(&id).copied().unwrap_or_default()
    }

    /// Returns `true` if `self` contains at least `value` amount of the asset with the given `id`.
    #[inline]
    pub fn contains(&self, id: AssetId, value: AssetValue) -> bool {
        self.balance(id) >= value
    }

    /// Deposit `asset` into `self`.
    #[inline]
    pub fn deposit(&mut self, asset: Asset) {
        *self.map.entry(asset.id).or_default() += asset.value;
    }

    /// Withdraw `asset` from `self`, returning `false` if it would overdraw the balance.
    #[inline]
    pub fn withdraw(&mut self, asset: Asset) -> bool {
        if asset.value == 0 {
            true
        } else {
            self.map
                .get_mut(&asset.id)
                .map(move |balance| {
                    if let Some(result) = balance.checked_sub(asset.value) {
                        *balance = result;
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
        }
    }
}

/// Action Types
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, tag = "type")]
pub enum Action {
    /// No Action
    None,

    /// Public Deposit Action
    PublicDeposit,

    /// Public Withdraw Action
    PublicWithdraw,

    /// Mint Action
    Mint,

    /// Private Transfer Action
    PrivateTransfer,

    /// Reclaim Action
    Reclaim,
}

/// Action Distribution Probability Mass Function
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct ActionDistributionPMF<T = f64> {
    /// No Action Weight
    pub none: T,

    /// Public Deposit Action Weight
    pub public_deposit: T,

    /// Public Withdraw Action Weight
    pub public_withdraw: T,

    /// Mint Action Weight
    pub mint: T,

    /// Private Transfer Action Weight
    pub private_transfer: T,

    /// Reclaim Action Weight
    pub reclaim: T,
}

impl Default for ActionDistributionPMF {
    #[inline]
    fn default() -> Self {
        Self {
            none: 1.0,
            public_deposit: 1.0,
            public_withdraw: 1.0,
            mint: 1.0,
            private_transfer: 1.0,
            reclaim: 1.0,
        }
    }
}

impl From<ActionDistribution> for ActionDistributionPMF {
    #[inline]
    fn from(actions: ActionDistribution) -> Self {
        ActionDistributionPMF {
            none: actions.distribution.pmf(0),
            public_deposit: actions.distribution.pmf(1),
            public_withdraw: actions.distribution.pmf(2),
            mint: actions.distribution.pmf(3),
            private_transfer: actions.distribution.pmf(4),
            reclaim: actions.distribution.pmf(5),
        }
    }
}

/// Action Distribution
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(
    deny_unknown_fields,
    into = "ActionDistributionPMF",
    try_from = "ActionDistributionPMF"
)]
pub struct ActionDistribution {
    /// Distribution over Actions
    distribution: Categorical,
}

impl Default for ActionDistribution {
    #[inline]
    fn default() -> Self {
        Self::try_from(ActionDistributionPMF::default()).unwrap()
    }
}

impl TryFrom<ActionDistributionPMF> for ActionDistribution {
    type Error = StatsError;

    #[inline]
    fn try_from(pmf: ActionDistributionPMF) -> Result<Self, StatsError> {
        Ok(Self {
            distribution: Categorical::new(&[
                pmf.none,
                pmf.public_deposit,
                pmf.public_withdraw,
                pmf.mint,
                pmf.private_transfer,
                pmf.reclaim,
            ])?,
        })
    }
}

impl Distribution<Action> for ActionDistribution {
    #[inline]
    fn sample<R>(&self, rng: &mut R) -> Action
    where
        R: RngCore + ?Sized,
    {
        match self.distribution.sample(rng) as usize {
            0 => Action::None,
            1 => Action::PublicDeposit,
            2 => Action::PublicWithdraw,
            3 => Action::Mint,
            4 => Action::PrivateTransfer,
            5 => Action::Reclaim,
            _ => unreachable!(),
        }
    }
}

/// User Account
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    /// Public Balances
    pub public: BalanceState,

    /// Secret Balances
    pub secret: BalanceState,

    /// Action Distribution
    pub actions: ActionDistribution,
}

impl Account {
    /// Samples a new account sampled using `config` settings and `rng`.
    #[inline]
    pub fn sample<R>(config: &Config, rng: &mut R) -> Self
    where
        R: RngCore + ?Sized,
    {
        let mut public = BalanceState::default();
        // TODO: Use a better distribution to sample a starting balance.
        for _ in 0usize..rng.gen_range(0..50) {
            public.deposit(config.sample_asset(rng));
        }
        Self {
            public,
            secret: Default::default(),
            actions: ActionDistribution::try_from(ActionDistributionPMF {
                none: rng.gen_range(config.action_sampling_ranges.none.clone()),
                public_deposit: rng.gen_range(config.action_sampling_ranges.public_deposit.clone()),
                public_withdraw: rng
                    .gen_range(config.action_sampling_ranges.public_withdraw.clone()),
                mint: rng.gen_range(config.action_sampling_ranges.mint.clone()),
                private_transfer: rng
                    .gen_range(config.action_sampling_ranges.private_transfer.clone()),
                reclaim: rng.gen_range(config.action_sampling_ranges.reclaim.clone()),
            })
            .unwrap(),
        }
    }
}

/// Simulation Update
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, tag = "type")]
pub enum Update {
    /// Create Account
    CreateAccount { account: Account },

    /// Deposit Public Balance
    PublicDeposit { account_index: usize, asset: Asset },

    /// Withdraw Public Balance
    PublicWithdraw { account_index: usize, asset: Asset },

    /// Mint Asset
    Mint { source_index: usize, asset: Asset },

    /// Private Transfer Asset
    PrivateTransfer {
        sender_index: usize,
        receiver_index: usize,
        asset: Asset,
    },

    /// Reclaim Asset
    Reclaim { sender_index: usize, asset: Asset },
}

/// Simulation Configuration
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Number of starting accounts
    pub starting_account_count: u64,

    /// Number of simulation steps before creating new accounts
    pub new_account_sampling_cycle: u64,

    /// [`Poisson`] growth rate of the number of accounts
    ///
    /// This configuration setting is not used if `new_account_sampling_cycle == 0`.
    pub account_count_growth_rate: f64,

    /// Maximum number of accounts
    ///
    /// If this value is less than `starting_account_count`, the maximum count is ignored.
    pub maximum_account_count: u64,

    /// Which assets are allowed to be sampled and the maximum per sample
    pub allowed_asset_sampling: IndexMap<AssetId, AssetValue>,

    /// Action Sampling Ranges
    ///
    /// This is a distribution over an [`ActionDistribution`] which is used to sample an
    /// [`ActionDistribution`] for a particular account.
    pub action_sampling_ranges: ActionDistributionPMF<Range<f64>>,

    /// Maximum number of updates allowed per step
    ///
    /// If this value is `0`, it has no effect.
    pub maximum_updates_per_step: u32,

    /// Maximum number of total updates
    ///
    /// If this value is `0`, it has no effect.
    pub maximum_total_updates: u32,
}

impl Config {
    /// Returns `true` if `self` has an active account count maximum.
    #[inline]
    fn has_maximum_account_count(&self) -> bool {
        self.maximum_account_count >= self.starting_account_count
    }

    /// Returns `true` if `accounts` is equal to the account count maximum, if it is active.
    #[inline]
    fn maximum_account_count_has_been_reached(&self, accounts: u64) -> bool {
        self.has_maximum_account_count() && self.maximum_account_count == accounts
    }

    /// Returns `true` if new accounts should be created for the current `step_counter` and an
    /// account list with `accounts`-many elements.
    #[inline]
    fn should_create_new_accounts(&self, step_counter: u64, accounts: u64) -> bool {
        self.maximum_account_count != self.starting_account_count
            && !self.maximum_account_count_has_been_reached(accounts)
            && self.new_account_sampling_cycle != 0
            && step_counter % self.new_account_sampling_cycle == 0
    }

    /// Samples an allowed asset using `rng`.
    #[inline]
    fn sample_asset<R>(&self, rng: &mut R) -> Asset
    where
        R: RngCore + ?Sized,
    {
        let id = self.sample_asset_id(rng);
        Asset::new(id, self.sample_asset_value(id, rng))
    }

    /// Samples an allowed asset id using `rng`.
    #[inline]
    fn sample_asset_id<R>(&self, rng: &mut R) -> AssetId
    where
        R: RngCore + ?Sized,
    {
        let mut ids = self.allowed_asset_sampling.keys();
        *ids.nth(rng.gen_range(0..ids.len())).unwrap()
    }

    /// Samples an allowed asset value of the given `id` using `rng`.
    #[inline]
    fn sample_asset_value<R>(&self, id: AssetId, rng: &mut R) -> AssetValue
    where
        R: RngCore + ?Sized,
    {
        rng.gen_range(0..=self.allowed_asset_sampling[&id])
    }

    /// Samples an allowed withdraw from `balances`.
    #[inline]
    fn sample_withdraw<R>(&self, balances: &BalanceState, rng: &mut R) -> Asset
    where
        R: RngCore + ?Sized,
    {
        let mut ids = self.allowed_asset_sampling.keys().collect::<Vec<_>>();
        ids.shuffle(rng);
        for id in &ids {
            let balance = balances.balance(**id);
            if balance != 0 {
                return Asset::new(
                    **id,
                    rng.gen_range(1..=min(balance, self.allowed_asset_sampling[*id])),
                );
            }
        }
        Asset::new(*ids[ids.len() - 1], 0)
    }
}

/// Simulator
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Simulator {
    /// Configuration
    config: Config,

    /// Step Counter
    step_counter: u64,

    /// Accounts
    accounts: Vec<Account>,
}

impl Simulator {
    /// Builds a new [`Simulator`] from the given `config`, sampling from `rng`.
    #[inline]
    pub fn new<R>(config: Config, rng: &mut R) -> Self
    where
        R: RngCore + ?Sized,
    {
        Self {
            accounts: (0..config.starting_account_count)
                .map(|_| Account::sample(&config, rng))
                .collect(),
            step_counter: Default::default(),
            config,
        }
    }

    /// Computes one step of the simulation using `rng`.
    #[inline]
    pub fn step<R>(&self, rng: &mut R) -> Vec<Update>
    where
        R: RngCore + ?Sized,
    {
        let mut updates = Vec::new();
        for (i, account) in self.accounts.iter().enumerate() {
            match account.actions.sample(rng) {
                Action::None => {}
                Action::PublicDeposit => {
                    updates.push(Update::PublicDeposit {
                        account_index: i,
                        asset: self.config.sample_asset(rng),
                    });
                }
                Action::PublicWithdraw => {
                    updates.push(Update::PublicWithdraw {
                        account_index: i,
                        asset: self.config.sample_withdraw(&account.public, rng),
                    });
                }
                Action::Mint => {
                    updates.push(Update::Mint {
                        source_index: i,
                        asset: self.config.sample_withdraw(&account.public, rng),
                    });
                }
                Action::PrivateTransfer => {
                    let asset = self.config.sample_withdraw(&account.secret, rng);
                    if asset.value > 0 {
                        updates.push(Update::PrivateTransfer {
                            sender_index: i,
                            receiver_index: rng.gen_range(0..self.accounts.len()),
                            asset
                        });
                    };
                }
                Action::Reclaim => {
                    updates.push(Update::Reclaim {
                        sender_index: i,
                        asset: self.config.sample_withdraw(&account.secret, rng),
                    });
                }
            }
        }
        let accounts_len = self.accounts.len() as u64;
        if self
            .config
            .should_create_new_accounts(self.step_counter, accounts_len)
        {
            let mut new_accounts = Poisson::new(self.config.account_count_growth_rate)
                .unwrap()
                .sample(rng) as u64;
            if self.config.has_maximum_account_count() {
                new_accounts =
                    new_accounts.clamp(0, self.config.maximum_account_count - accounts_len);
            }
            for _ in 0..new_accounts {
                updates.push(Update::CreateAccount {
                    account: Account::sample(&self.config, rng),
                });
            }
        }
        if self.config.maximum_updates_per_step > 0 {
            choose_multiple(
                &mut updates,
                self.config.maximum_updates_per_step as usize,
                rng,
            );
        }
        updates
    }

    /// Applies `update` to the internal state of the simulator, returning the update back
    /// if an error occured.
    #[inline]
    pub fn apply(&mut self, update: Update) -> Result<(), Update> {
        match &update {
            Update::CreateAccount { account } => {
                self.accounts.push(account.clone());
                return Ok(());
            }
            Update::PublicDeposit {
                account_index,
                asset,
            } => {
                if let Some(balances) = self.accounts.get_mut(*account_index) {
                    balances.public.deposit(*asset);
                    return Ok(());
                }
            }
            Update::PublicWithdraw {
                account_index,
                asset,
            } => {
                if let Some(balances) = self.accounts.get_mut(*account_index) {
                    if balances.public.withdraw(*asset) {
                        return Ok(());
                    }
                }
            }
            Update::Mint {
                source_index,
                asset,
            } => {
                if let Some(balances) = self.accounts.get_mut(*source_index) {
                    if balances.public.withdraw(*asset) {
                        balances.secret.deposit(*asset);
                        return Ok(());
                    }
                }
            }
            Update::PrivateTransfer {
                sender_index,
                receiver_index,
                asset,
            } => {
                if let Some(sender) = self.accounts.get_mut(*sender_index) {
                    if sender.secret.withdraw(*asset) {
                        if let Some(receiver) = self.accounts.get_mut(*receiver_index) {
                            receiver.secret.deposit(*asset);
                            return Ok(());
                        }
                    }
                }
            }
            Update::Reclaim {
                sender_index,
                asset,
            } => {
                if let Some(balances) = self.accounts.get_mut(*sender_index) {
                    if balances.secret.withdraw(*asset) {
                        balances.public.deposit(*asset);
                        return Ok(());
                    }
                }
            }
        }
        Err(update)
    }

    /// Runs `self` for the given number of `steps`.
    #[inline]
    pub fn run<R>(&mut self, steps: usize, rng: &mut R) -> Simulation
    where
        R: RngCore + ?Sized,
    {
        let initial_accounts = self.accounts.clone();
        let mut updates = Vec::new();
        for _ in 0..steps {
            let mut next_updates = self.step(rng);
            let update_limit = self.config.maximum_total_updates as usize;
            if update_limit > 0 {
                match update_limit - updates.len() {
                    0 => break,
                    diff => next_updates.truncate(diff),
                }
            }
            for update in &next_updates {
                if let Err(update) = self.apply(update.clone()) {
                    panic!(
                        "ERROR: {}\n\n Panicked on the following state:\nSimulation: {:?}\nUpdate: {:?}",
                        "This is an internal simulation error. Please file a bug.",
                        self,
                        update
                    );
                }
            }
            updates.append(&mut next_updates);
            self.step_counter += 1;
        }
        Simulation {
            config: self.config.clone(),
            initial_accounts,
            final_accounts: self.accounts.clone(),
            updates,
        }
    }
}

/// Simulation Final State
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Simulation {
    /// Configuration
    pub config: Config,

    /// Initial Account State
    pub initial_accounts: Vec<Account>,

    /// Final Account State
    pub final_accounts: Vec<Account>,

    /// Updates
    pub updates: Vec<Update>,
}

/// Runs a [`Simulator`] in a CLI.
pub fn main() {
    let matches = App::new("Manta Simulation")
        .arg(
            Arg::with_name("steps")
                .help("The number of steps to run the simulation.")
                .required(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file. By default, `default-config.json` is used.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets a custom output file")
                .takes_value(true),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("default-config.json");
    let config = match fs::read_to_string(&config_path) {
        Ok(config) => match serde_json::from_str(&config) {
            Ok(config) => config,
            err => panic!("ERROR: {:?}", err),
        },
        _ => panic!("ERROR: Invalid configuration path: {:?}", config_path),
    };

    let steps = matches
        .value_of("steps")
        .unwrap()
        .parse::<usize>()
        .expect("ERROR: Invalid number of simulation steps.");

    let mut rng = thread_rng();

    print!("INFO: Running simulation ... ");
    flush_stdout();
    let simulation = Simulator::new(config, &mut rng).run(steps, &mut rng);
    println!("DONE.");

    if let Some(output_path) = matches.value_of("output") {
        print!("INFO: Writing simulation to file ... ");
        flush_stdout();
        match serde_json::to_writer(
            fs::File::create(output_path).expect("ERROR: Unable to create output file."),
            &simulation,
        ) {
            Ok(()) => println!("DONE. Output written to `{}`.", output_path),
            err => panic!("ERROR: {:?}", err),
        }
    } else if let Err(err) = serde_json::to_writer_pretty(std::io::stdout(), &simulation) {
        panic!("ERROR: {:?}", err);
    }
}
