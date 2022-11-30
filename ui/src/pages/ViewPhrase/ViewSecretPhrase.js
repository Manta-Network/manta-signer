import { useState } from "react";
import ViewPhrasePage from "./ViewPhrasePage";
import ViewPhraseSuccess from "./ViewPhraseSuccess";

const ViewSecretPhrase = ({
  endExportPhrase,
  exportedSecretPhrase,
  sendPassword,
  stopPasswordPrompt
}) => {

  const [password, setPassword] = useState("");
  const [passwordInvalid, setPasswordInvalid] = useState(false);
  const [recoveryPhraseConfirmed, setRecoveryPhraseConfirmed] = useState(false);
  const [loading, setLoading] = useState(false);

  const onChangePassword = password => {
    setPassword(password)
    setPasswordInvalid(false)
  }

  const onClickCancel = async () => {
    await stopPasswordPrompt();
    endExportPhrase();
  }

  const onClickSubmitPassword = async () => {
    setLoading(true);
    const shouldRetry = await sendPassword(password);

    if (!shouldRetry) {
      setPassword('');
      setPasswordInvalid(false);
    } else {
      console.log("[INFO]: Invalid password, RETRY!");
      setPasswordInvalid(true);
      setLoading(false);
    }
  }

  // This function enables the Next button to continue in the account creation
  // process once the user has read the recovery phrase.
  const onClickConfirmRecoveryPhrase = () => {
    setRecoveryPhraseConfirmed(true);
  }

  const onClickFinish = () => {
    setRecoveryPhraseConfirmed(false);
    endExportPhrase();
    setLoading(false);
  }

  return (<>
    {!exportedSecretPhrase && (
      <ViewPhrasePage
      loading={loading}
      exportedSecretPhrase={exportedSecretPhrase}
      password={password}
      passwordInvalid={passwordInvalid}
      onChangePassword={onChangePassword}
      onClickSubmitPassword={onClickSubmitPassword}
      onClickCancel={onClickCancel}
      />
    )}
    {exportedSecretPhrase &&
    <ViewPhraseSuccess
    recoveryPhraseConfirmed={recoveryPhraseConfirmed}
    exportedSecretPhrase={exportedSecretPhrase}
    onClickConfirmRecoveryPhrase={onClickConfirmRecoveryPhrase}
    onClickFinish={onClickFinish}
    />}
  </>);
}

export default ViewSecretPhrase