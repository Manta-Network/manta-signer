import { Button, Input, Form, Dropdown } from 'semantic-ui-react';
import HyperLinkButton from '../../components/HyperLinkButton';
import "../../App.css";
import { useOutletContext } from 'react-router-dom';

const SeedPhrase = () => {

  const {
    onChangeDropDown,
    onChangeWord,
    goBack,
    goForward,
    DROPDOWN_OPTIONS,
    mnemonicsValidity,
    mnemonics }
    = useOutletContext();

  return (<>

    <div className='recoverHeaderContainer'>
      <h1 className='mainheadline'>Reset Wallet</h1>
      <p className='subtext'>
        You can reset your password by entering your secret recovery phrase.
      </p>
    </div>

    <div>
      <Dropdown
        className="ui fluid dropdown compressed"
        fluid
        selection
        options={DROPDOWN_OPTIONS}
        onChange={onChangeDropDown}
        defaultValue={DROPDOWN_OPTIONS[0].value}
      />
    </div>

    <Form className="ui form adjusted">
      {mnemonics.map(function (_item, index) {
        return (
          <Form.Field
            className="ui form field thin"
            placeholder={(index + 1).toString() + "."}
            control={Input}
            key={index}
            onChange={(e, textObj) => onChangeWord(e, textObj, index)}
          />
        )
      })}
    </Form>

    <div>
      {mnemonicsValidity ?
        <Button primary className="button ui first" onClick={goForward}>Next</Button> :
        <Button disabled primary className="button ui first">Next</Button>}
    </div>
    <HyperLinkButton
      text={"Go Back"}
      onclick={goBack}
    />
  </>)
};

export default SeedPhrase;