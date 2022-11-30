import { Button, Icon } from 'semantic-ui-react';
import "../App.css";

const CopyButton = ({
  showCopyNotification,
  onClickCopyPhrase
}) => {
  return (
    <>
      {
        showCopyNotification ?
          <Button onClick={onClickCopyPhrase} className='button ui copy'>
            <Icon name="checkmark" className='specific' />
          </Button>
          :
          <Button onClick={onClickCopyPhrase} className='button ui copy'>
            <Icon name="copy outline" className='specific' />
          </Button>
      }
    </>
  )
}

export default CopyButton;