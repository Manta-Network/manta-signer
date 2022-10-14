import { Button } from 'semantic-ui-react';
import "../App.css";

const HyperLinkButton = ({
  onclick,
  text
}) => {

  return (
    <div className='hrefButtonContainer'>
      <Button className='button ui hrefButton' onClick={onclick}>{text}</Button>
    </div>
  )
}

export default HyperLinkButton;