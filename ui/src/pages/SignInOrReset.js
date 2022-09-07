import { Button, Input, Label, Header } from 'semantic-ui-react';

const SignInOrReset = (props) => {


  const onClickStartSignIn = async () => {
    props.startSignIn();
  }
  
  const onClickStartReset = async () => {
    props.startReset();
  }

  return (<>
    <Header>
      Sign in or reset your account
    </Header>

    <Button primary className="button" onClick={onClickStartSignIn}>Sign In</Button>
    <Button secondary className="button" onClick={onClickStartReset}>Reset</Button>

  </>);
}

export default SignInOrReset;