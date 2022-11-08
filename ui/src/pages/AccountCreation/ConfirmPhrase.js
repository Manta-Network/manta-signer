import { Button } from 'semantic-ui-react';
import HyperLinkButton from '../../components/HyperLinkButton';
import "../../App.css";
import { useOutletContext } from 'react-router-dom';

const ConfirmPhrase = () => {

  const {
    goBack,
    goForward,
    onClickSelectWordButton,
    selectedRecoveryPhrase,
    shuffledRecoveryPhrase,
    isValidSelectedPhrase
  } = useOutletContext();

  return (<>
    <div className='tall-header-container'>
      <h1 className='main-headline'>Confirm Your Secret Recovery Phrase</h1>
      <p className='sub-text'>
        Please select the appropriate phrase in the correct order, from left to right.
      </p>
    </div>

    <div className='word-list-container'>
      {selectedRecoveryPhrase.map(function (item, index) {
        let word = item.split("_")[0];
        let idx = index + 1;
        return <div className='button ui button-list' key={index}>
          <table className='numbered-table-word'>
            <tbody>
              <tr>
                <th><h4 className='numbered-word-selection'>{idx + "."}&nbsp;</h4></th>
                <th><h4>{word}</h4></th>
              </tr>
            </tbody>
          </table>
        </div>
      })}
    </div>

    <div className='button-list-container'>
      {shuffledRecoveryPhrase.map(function (item) {
        // in this case `item` consists of the word followed by an index to make it a unqiue element.
        // however, `word` is a string that consists of just the word.
        let word = item.split("_")[0];
        return (
          <Button
            onClick={(e) => onClickSelectWordButton(e, item)}
            className="button ui button-list"
            key={item}>
            {word}
          </Button>
        )
      })}
    </div>
    <Button disabled={!isValidSelectedPhrase} className="button ui first wide" onClick={goForward}>
      Confirm
    </Button>
    <HyperLinkButton
      text={"Go Back"}
      onclick={goBack}
    />

  </>)
}

export default ConfirmPhrase;