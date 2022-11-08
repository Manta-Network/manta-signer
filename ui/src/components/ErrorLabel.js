import { Label, Icon } from "semantic-ui-react"


const ErrorLabel = ({
  text
}) => {
  return (
    <div className="error-label-container">
      <Label className="ui basic label error-label">
        <Icon name="exclamation circle" className='specific' />
        <p>{text}</p>
      </Label>
    </div>
  )
}

export default ErrorLabel;