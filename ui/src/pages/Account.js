import { Button, Header } from 'semantic-ui-react';


const Account = ({ onResetAccount, hideWindow }) => {
  const onClickConfirmReset = async () => {
      await onResetAccount();
  };

  const onClickClose = async () => {
      await hideWindow();
  };

  return (
    <>
      <Header className="recovery-phrase-header">
        Account Info
      </Header>
      <Button className="button" onClick={onClickConfirmReset}>
          Reset Account
      </Button>
      <Button className="button" onClick={onClickClose}>
          Close
      </Button>
    </>
  )
};

export default Account;
