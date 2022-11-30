import { Button } from 'semantic-ui-react';
import "../App.css";

const HyperLinkButton = ({
  onclick,
  text
}) => {

  return (
    <div className='href-button-container'>
      <Button className='button ui href-button' onClick={onclick}>{text}</Button>
    </div>
  )
}

export default HyperLinkButton;