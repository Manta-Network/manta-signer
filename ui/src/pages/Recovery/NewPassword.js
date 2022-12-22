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
      <div className='header-container padded-bottom-3rem'>
        <h1 className='main-headline'>Create a password</h1>
        <p className='sub-text'>Your password will unlock the Manta Signer.</p>
      </div>
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
          <ErrorLabel
            text={"Please enter a minimum of " + MIN_PASSWORD_LENGTH + " characters."}
          />
        </>
        :
        (
          !passwordsMatch && showError ?
            <>
              <ErrorLabel
                text={"Passwords do not match."}
              />
            </>
            :
            <div className='empty-place-holder-4rem'></div>
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