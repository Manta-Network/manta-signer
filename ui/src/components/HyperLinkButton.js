import { Button } from 'semantic-ui-react';
import "../App.css";

const HyperLinkButton = (props) => {

  return (
    <div className='hrefButtonContainer'>
      <Button className='button ui hrefButton' onClick={props.goBack}>Go Back</Button>
    </div>
  )
}

export default HyperLinkButton;