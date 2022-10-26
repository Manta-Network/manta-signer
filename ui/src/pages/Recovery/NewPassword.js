import { Button, Input } from 'semantic-ui-react';
import HyperLinkButton from '../../components/HyperLinkButton';
import "../../App.css";
import { useOutletContext } from 'react-router-dom';
import ErrorLabel from '../../components/ErrorLabel';

const NewPassword = () => {

  const {
    goBack,
    checkPasswords,
    onChangePassword,
    onChangeConfirmPassword,
    MIN_PASSWORD_LENGTH,
    isValidPassword,
    passwordsMatch,
    password,
    showError
  } = useOutletContext();


  return (
    <>
      <div className='headercontainer'>
        <h1 className='mainheadline'>Create a password</h1>
        <p className='subtext'>Your password will unlock the Manta Signer.</p>
      </div>
      <br />
      <br />
      <div>
        <Input
          className='input ui password'
          type="password"
          placeholder="Password (8 characters min)"
          onChange={(e) => onChangePassword(e)}
        />
      </div>
      <div>
        <Input
          className='input ui password'
          type="password"
          placeholder="Confirm Password"
          onChange={(e) => onChangeConfirmPassword(e)}
        />
      </div>
      {!isValidPassword && password.length > 0 && showError ?
        <>
          <br />
          <ErrorLabel
            text={"Please enter a minimum of " + MIN_PASSWORD_LENGTH + " characters."}
          />
        </>
        :
        (
          !passwordsMatch && showError ?
            <>
              <br />
              <ErrorLabel
                text={"Passwords do not match."}
              />
            </>
            :
            <><br /><br /><br /></>
        )}
      <Button className="button ui first"
        onClick={checkPasswords}>
        Next
      </Button>
      <HyperLinkButton
        text={"Go Back"}
        onclick={goBack}
      />

    </>
  )
}

export default NewPassword;