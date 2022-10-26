import { Label, Icon } from "semantic-ui-react"


const ErrorLabel = ({
  text
}) => {
  return (
    <div className="errorLabelContainer">
      <Label className="ui basic label errorlabel">
        <Icon name="exclamation circle" className='specific' />
        <p>{text}</p>
      </Label>
    </div>
  )
}

export default ErrorLabel;