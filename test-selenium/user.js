const {Builder, Capabilities, By, Key, navigator, until} = require('selenium-webdriver');

// derivation paths found here: https://docs.substrate.io/v3/tools/subkey/#well-known-keys
const DEV_ACCOUNT_SEED = 'bottom drive obey lake curtain smoke basket hold race lonely fit walk';
const TEST_ACCT_PASS = 'test_password';

// TODO: changeme
const POLKADOT_PLUGIN_PATH = '/home/tj/manta/test-selenium/0.42.2_0/';
const BALANCE_SYNC_TEXT = '...';

// Click the buttons!!
const UNDERSTOOD_BTN = "Understood, let me continue";
const YES_ALLOW_BTN = "Yes, allow this application access";
const ADD_THE_ACCOUNT = "Add the account with the supplied seed";

const sleep = (seconds) => { return new Promise(r => setTimeout(r, seconds * 1000)); };

const DEBUG = process.env.DEBUG || false;

// NOTE: these should match the UI list ordering, maybe should import from there
const PRIVATE_ASSETS = [
    "pKAR",
    "paUSD",
    "pKSM",
    "pROC",
    "pkBTC",
    "pMOVR",
    "pDOL",
];

const PUBLIC_ASSETS = [
    "KAR",
    "aUSD",
    "KSM",
    "ROC",
    "kBTC",
    "MOVR",
    "DOL",
];

function debug_print(text) {
    if (DEBUG) {
        console.log(text);
    }
}


class CryptoPerson {
    constructor(driver, derivation_path) {
        this.path = derivation_path;
        this.driver = driver;
    }

    static async initialize(derivation_path, port) {
        const chromabilities = Capabilities.chrome();

        chromabilities.set("goog:chromeOptions", {
            args: ['--window-size=1920,1080', `--load-extension=${POLKADOT_PLUGIN_PATH}`]
        });

        let driver = await new Builder()
            .forBrowser("chrome")
            .withCapabilities(chromabilities)
            .build();

        const me = new CryptoPerson(driver, derivation_path);

        await me.setup_wallet_account();
        debug_print(`Account for ${derivation_path} initialized`);

        await driver.get(`http://localhost:${port}/#/transact`);

        const parent_window = await switch_to_child(driver);
    
        await understood_lets_go(driver);
    
        await yes_allow(driver);
    
        debug_print(`Back to the parent's world: ${parent_window}`);
        await back_to_parent(driver, parent_window);

        debug_print("Waiting for wallet sync");
        await me.await_sync();

        await me.toggle_to_public("sender");

        let pub_account = await driver.findElement(By.id("clipBoardCopy"));
        me.public_account = await pub_account.getAttribute("value");
        debug_print(`Public account is: ${me.public_account}`);

        await me.toggle_to_private("sender");

        let priv_account = await driver.findElement(By.id("clipBoardCopy"));
        me.private_account = await priv_account.getAttribute("value");
        debug_print(`Private account is: ${me.private_account}`);

        debug_print("Waiting for private balance sync");
        await me.await_sync();

        debug_print(`${derivation_path} is ready to transact!`);
        return me;
    }

    async setup_wallet_account() {
        await this.driver.get("chrome-extension://mopnmbcafieddcagagdcbnhejhlodfdd/index.html#/account/import-seed");

        debug_print(`Importing account for ${this.acct_path}`);
        let text_area = await this.driver.findElements(By.tagName("textarea"));

        // should only be one of these.
        for (let area of text_area) {
            await area.sendKeys(DEV_ACCOUNT_SEED);
        }

        let advanced_toggle = await this.driver.findElements(By.className('advancedToggle'));

        for (let togg of advanced_toggle) {
            await togg.click();
            break;
        }

        let path_input = await this.driver.findElements(By.tagName("input"));

        for (let inp of path_input) {
            await inp.sendKeys(this.path);
            break;
        }

        let next_button = await this.driver.findElements(By.tagName("button"));

        for (let btn of next_button) {
            await btn.click();
            break;
        }

        let inputs = await this.driver.findElements(By.tagName("input"));
        let idx = 0;
        for (let inp of inputs) {
            if (idx == 0) {
                await inp.sendKeys(this.path);
            } else if (idx == 1) {
                await inp.sendKeys(TEST_ACCT_PASS);
            } else {
                debug_print("ERROR: too many inputs");
                process.exit(1);
            }
            idx += 1;
        }

        inputs = await this.driver.findElements(By.tagName("input"));
        idx = 0;
        for (let inp of inputs) {
            if (idx == 2) {
                await inp.sendKeys(TEST_ACCT_PASS);
            } else if (idx > 2) {
                debug_print("ERROR: too many inputs");
                process.exit(1);
            }
            idx += 1;
        }

        let btns = await this.driver.findElements(By.tagName("button"));

        for (let btn of btns) {
            let txt = await btn.getText();
            if (txt == ADD_THE_ACCOUNT) {
                await btn.click();
                break;
            }
        }

        await understood_lets_go(this.driver);
        debug_print(`Account for ${this.path} added`);
    }

    async set_recipient(who) {
        let recipient = await this.driver.findElement(By.id("recipientAddress"));
        await recipient.sendKeys(who);
    }

    async public_asset_select(asset) {
        // check if we're on the right page, exception raises if not.
        await this.driver.findElement(By.id('senderPublicTogglePrivate'));

        let assetIndex = PUBLIC_ASSETS.indexOf(asset);
        if (assetIndex == -1) {
            throw new Error(`Asset ${asset} does not exist`);
        }

        const happn = await this.driver.findElement(By.id("selectedAssetType"));
        await happn.click();

        const options = await happn.findElements(By.css("*"));
        for (let opt of options) {
            let id = await opt.getAttribute("id");
            let txt = await opt.getText();
            if (id.startsWith('react-select')) {
                let splitted = id.split('-');
                let idx = parseInt(splitted[splitted.length - 1]);
                if (idx == assetIndex) {
                    debug_print(`Clicking on ${txt}`);
                    await opt.click();
                    break;
                }
            }
        }
    }

    async private_asset_select(asset) {
        // check if we're on the right page, exception raises if not.
        await this.driver.findElement(By.id('senderPrivateTogglePublic'));

        let assetIndex = PRIVATE_ASSETS.indexOf(asset);
        if (assetIndex == -1) {
            throw new Error(`Asset ${asset} does not exist`);
        }

        const happn = await this.driver.findElement(By.id("selectedAssetType"));
        await happn.click();

        const options = await happn.findElements(By.css("*"));
        for (let opt of options) {
            let id = await opt.getAttribute("id");
            let txt = await opt.getText();
            if (id.startsWith('react-select')) {
                let splitted = id.split('-');
                let idx = parseInt(splitted[splitted.length - 1]);
                if (idx == assetIndex) {
                    debug_print(`Clicking on ${txt}`);
                    await opt.click();
                    break;
                }
            }
        }
    }

    async send(amount, expected) {
        const amt = await this.driver.findElement(By.id("amountInput"));
        await amt.sendKeys(amount);

        await sleep(1);

        const sendBtn = await this.driver.findElement(By.id("sendButton"));
        await sendBtn.click();

        const parent_window = await switch_to_child(this.driver);

        const inputs = await this.driver.findElements(By.tagName("input"));
        for (let inp of inputs) {
            let attr = await inp.getAttribute("type");
            if (attr == "password") {
                await inp.sendKeys(TEST_ACCT_PASS);
                break;
            }
        }

        const approveBtn = await this.driver.findElement(By.tagName("button"));

        debug_print(`Clicking on ${await approveBtn.getText()}`);
        await approveBtn.click();

        await back_to_parent(this.driver, parent_window);

        let notification_recv = false;
        while (!notification_recv) {
            try {
                let notif = await this.driver.findElement(By.className('el-notification'));
                let result = await notif.getText();

                if (!result.includes(expected)) {
                    throw Error(`Transaction did not complete as expected: ${result} != ${expected}`);
                }

                notification_recv = true;
            } catch (e) {
                if (e.name == 'NoSuchElementError') {
                    debug_print(`No notify yet...`);
                    await sleep(1);
                } else {
                    throw e;
                }
            }
        }
    }

    async toggle_to_public(who) {
        try {
            let toggle = await this.driver.findElement(By.id(`${who}PrivateTogglePublic`));

            await this.driver.wait(until.elementIsVisible(toggle));
            await toggle.click();
        } catch (e) {
            if (e.name == 'NoSuchElementError') {
                debug_print(`Already on public ${who}`);
            } else {
                throw e;
            }
        }

        debug_print("Should be on private page now");
    }

    async toggle_to_private(who) {
        try {
            let toggle = await this.driver.findElement(By.id(`${who}PublicTogglePrivate`));

            await this.driver.wait(until.elementIsVisible(toggle));
            await toggle.click();
        } catch (e) {
            if (e.name == 'NoSuchElementError') {
                debug_print(`Already on private ${who}`);
            } else {
                throw e;
            }
        }

        debug_print("Should be on private page now");
    }

    async await_sync() {
        let synced = false;
        let acct_balance;

        while (!synced) {
            let balance = await this.driver.findElement(By.id("balanceText"));

            let bal = await balance.getText();
            debug_print(`Balance text: ${bal}`);
            if ( bal.endsWith(BALANCE_SYNC_TEXT) ) {
                debug_print("Waiting for balance to sync...");
                await sleep(5);
            } else {
                acct_balance = bal.split(' ')[1];
                synced = true;
            }
        }

        return acct_balance;
    }

    async public_balance(asset) {
        await this.toggle_to_public("sender");
        await this.public_asset_select(asset);
        const balance = await this.await_sync();
        return balance;
    }

    async private_balance(asset) {
        await this.toggle_to_private("sender");
        await this.private_asset_select(asset);
        const balance = await this.await_sync();
        return balance;
    }

    async private_private_send(to, asset, amount, expected) {
        await this.toggle_to_private("sender");
        await this.toggle_to_private("receiver");
        await this.private_asset_select(asset);
        await this.set_recipient(to);
        await this.send(amount, expected);
    }

    async public_private_send(to, asset, amount, expected) {
        await this.toggle_to_public("sender");
        await this.toggle_to_private("receiver");
        await this.public_asset_select(asset);
        // Send to self
        //await this.set_recipient(to);
        await this.send(amount, expected);
    }

    async private_public_send(to, asset, amount, expected) {
        await this.toggle_to_private("sender");
        await this.toggle_to_public("receiver");
        await this.private_asset_select(asset);
        // Withdraw to self
        //await this.set_recipient(to);
        await this.send(amount, expected);
    }

    async public_public_send(to, asset, amount, expected) {
        await this.toggle_to_public("sender");
        await this.toggle_to_public("receiver");
        await this.public_asset_select(asset);
        await this.set_recipient(to);
        await this.send(amount, expected);
    }
}


async function back_to_parent(driver, window_id) {
    await driver.switchTo().window(window_id);
}


async function switch_to_child(driver) {
    const parent_window = await driver.getWindowHandle();

    let handles = await driver.getAllWindowHandles();
    while (handles.length == 1) {
        debug_print("Waiting for pop-up");
        await sleep(1);
        handles = await driver.getAllWindowHandles();
    }

    let child_window = handles.filter(handle => handle != parent_window)[0];

    debug_print(`Switching to: ${child_window}`);
    await driver.switchTo().window(child_window);

    return parent_window;
}



async function yes_allow(driver) {
    buttons = await driver.findElements(By.tagName("button"));
    for (let button of buttons) {
        let button_text = await button.getText();
        if (button_text == YES_ALLOW_BTN) {
            await button.click();
            break;
        }
    }
}


async function understood_lets_go(driver) {
    let buttons = await driver.findElements(By.tagName("button"));
    for (let button of buttons) {
        let button_text = await button.getText();
        if (button_text == UNDERSTOOD_BTN) {
            await button.click();
            break;
        }
    }
}


module.exports.CryptoPerson = CryptoPerson;
module.exports.PUBLIC_ASSETS = PUBLIC_ASSETS;
module.exports.PRIVATE_ASSETS = PRIVATE_ASSETS;
