import { Button } from 'semantic-ui-react';
import "../App.css";

const HyperLinkButton = (props) => {

  return (
    <div className='hrefButtonContainer'>
      <Button className='button ui hrefButton' onClick={props.onclick}>{props.text}</Button>
    </div>
  )
}

export default HyperLinkButton;