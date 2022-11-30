import { Button, Input, Form } from 'semantic-ui-react';
import HyperLinkButton from '../../components/HyperLinkButton';
import mantaLogo from "../../icons/manta.png";
import "../../App.css";

const SignInPage = ({
  loading,
  password,
  passwordInvalid,
  onChangePassword,
  onClickSignIn,
  onClickForgotPassword,
}) => {

  return (<div>
    <div className='main-logo-container login'>
      <img className="main-logo" alt="Manta Logo" src={mantaLogo} />
    </div>

    <div>
      <h1 className='main-headline'>Welcome Back!</h1>
      <p className='sub-text'>Let's connect you to the Web3 privacy hub.</p>
    </div>
    <Form.Field>
      <Input
        className='input ui password tall'
        type="password"
        placeholder="Password"
        value={password}
        onChange={(e) => onChangePassword(e.target.value)}
        error={passwordInvalid}
      />
    </Form.Field>
    {
      loading ?
        <Button disabled={true} className="button ui first">Loading</Button>
        :
        <Button className="button ui first" onClick={onClickSignIn}>
          Unlock
        </Button>
    }
    <HyperLinkButton
      text={"Forgot Password?"}
      onclick={onClickForgotPassword}
    />

    <div className='learn-about-manta-small'>
      <p>Learn more about &nbsp;
        <a href='https://www.manta.network/' target="_blank" rel="noreferrer">
          Manta
        </a>
      </p>
    </div>
  </div>)
}

export default SignInPage;