import { Button, Input, Label } from 'semantic-ui-react';
import HyperLinkButton from '../../components/HyperLinkButton';
import "../../App.css";
import { useOutletContext } from 'react-router-dom';

const NewPassword = () => {

  const {
    goBack,
    goForward,
    onChangePassword,
    onChangeConfirmPassword,
    MIN_PASSWORD_LENGTH,
    isValidPassword,
    passwordsMatch,
    password,
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
      <Button disabled={((!isValidPassword) || (!passwordsMatch))} className="button ui first"
        onClick={goForward}>
        Next
      </Button>
      <HyperLinkButton
        text={"Go Back"}
        onclick={goBack}
      />

      {!isValidPassword && password.length > 0 ? <><br /><Label basic color='red' pointing>Please enter a minimum of {MIN_PASSWORD_LENGTH} characters.</Label></> : (
        !passwordsMatch ? <><br /><Label basic color='red' pointing>Passwords do not match.</Label></> : <><br /><br /><br /></>
      )}

    </>
  )
}

export default NewPassword;