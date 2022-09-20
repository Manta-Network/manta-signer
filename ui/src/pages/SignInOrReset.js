import { Button, Input, Label, Header } from 'semantic-ui-react';
import mainLogo from "../icons/Square150x150Logo.png"
import "../App.css";

const SignInOrReset = (props) => {


  const onClickStartSignIn = async () => {
    props.startSignIn();
  }

  const onClickStartReset = async () => {
    props.startReset();
  }

  return (<>
    <div>
      <img draggable="false" unselectable="on" dragstart="false" src={mainLogo} />
    </div>

    <Button className="button ui first" onClick={onClickStartSignIn}>Log In</Button>
    <Button className="button ui two" onClick={onClickStartReset}>Reset</Button>

  </>);
}

export default SignInOrReset;