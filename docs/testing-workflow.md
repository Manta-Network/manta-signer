# Testing Workflow

To test the signer, we want to try and explore as much of the state machine as possible without visiting diverging cycles. Here is an overview of the `SERVER` and `UI` state machines:

```text
——————————
| SERVER |
——————————

1. [STARTUP]
    - If NO: Send "CREATE ACCOUNT" to UI. GOTO step 2.
    - If YES: Send "LOGIN" to UI. GOTO step 3.

2. [ACCOUNT CREATION]
    - WAIT for password from UI. Use password to generate mnemonic and send it back to UI.
    - GOTO step 4.

3. [LOGIN/AUTHORIZATION]
    - WAIT for password from UI. Check if password can unlock root seed.
    - IF YES: Send "SLEEP" message to UI. GOTO step 4.
    - IF NO: GOTO step 3.

4. [SERVE]
    - WAIT for front-end messages.
    - If message requires authorization, send "WAKE" to UI and GOTO step 3.
```

```text
——————
| UI |
——————

1. [STARTUP]
    - WAIT for startup message from SERVER.
    - If "CREATE ACCOUNT" GOTO step 2.
    - If "LOGIN" GOTO step 3.

2. [ACCOUNT CREATION]
    - Ask user for new password and send to SERVER.
    - WAIT for mnemonic from SERVER.
    - Show user mnemonic for wallet recovery.
    - After confirming that they have memorized it, HIDE UI.
    - GOTO step 4.

3. [LOGIN/AUTHORIZATION]
    - Ask user for password and send it to SERVER.
    - WAIT for retry message from SERVER saying if the password should be retried or not.
    - If YES: GOTO step 3.
    - If NO: HIDE UI.
    - GOTO step 4.

4. [LISTEN]
    - WAIT for SERVER "WAKE" messages. When message is received, GOTO step 3.
```

To test these machines, we want to enter every cycle at least once to visit every state, and at least twice to test if the state is stable under re-entries. It is possible that there are long-term deviations from this protocol present that can only be detected with a large number of cycle re-entries but this is unlikely. To confirm this, it is better to read the code and understand how these state machines are implemented. 

## Tests

For all of these tests make sure to have the signer ready:

- Development mode: `cargo tauri dev`
- Production mode: `cargo tauri build`

No need to run `yarn` commands, they are run automatically. Also, make sure to have a tab open in your browser to the frontend.

For development builds, you have access to the SERVER logs and the UI console which will detail step-by-step each state transition for each machine.

Below, when you are asked to QUIT the signer, simply go to the toolbar and click on the dolphin logo and press "Quit".

### Full Create Account Test

Starting with a clean slate we want to test everything through without logging in.

```text
1. OPEN signer.
2. ENSURE that the UI prompts you with a "CREATE ACCOUNT" screen.
3. INPUT password.
4. ENSURE that a mnemonic appears within a few seconds. Accept prompt.
5. ENSURE that the UI becomes hidden.
6. GOTO browser frontend. Submit a PUBLIC transaction.
8. ENSURE that you are NOT prompted by the UI.
9. GOTO browser frontend. Submit a MINT transaction.
10. ENSURE that you are NOT prompted by the UI. If in development mode, check to see that the SERVER is running MINT-related commands.
11. GOTO browser frontend. Submit a PRIVATE TRANSFER or RECLAIM transaction.
12. ENSURE that you are prompted by the UI with a transaction authorization page within a few seconds.
13. ENSURE that the transaction summary displays the same information as that from the browser frontend.

NOTE: At this point we can try three different transitions: 1. Accept with correct password, 2. Accept with incorrect password, 3. Decline transaction.

14. DO Accept with incorrect pasword.
15. ENSURE that the UI tells you it's wrong and reprompts for password.
16. CHOOSE: Accept with correct password or Decline transaction.
17. IF ACCEPT: ENSURE that the UI is hidden and that the proof generation begins. For dev builds it will take 10-100x the time so QUIT the signer at this point.
18. IF DECLINE: ENSURE that the UI is hidden and that the proof generation does NOT begin. GOTO browser frontend and see that the transaction failed.
19. CHOOSE: GOTO step 11 or QUIT signer.
20. STOP.
```

### Quick Create Account Test

To quickly test the account creation mechanism, do the following:

```text
1. OPEN signer.
2. ENSURE that the UI prompts you with a "CREATE ACCOUNT" screen.
3. INPUT password.
4. ENSURE that a mnemonic appears within a few seconds. Accept prompt.
5. ENSURE that window becomes hidden.
6. QUIT signer.
7. STOP.
```

### Login Test

After running one of the tests above, do the following:

1. OPEN signer.
2. ENSURE that the UI prompts you with a "LOGIN" screen.
3. INPUT incorrect password.
4. ENSURE that the UI tells you it's wrong and reprompts for password.
5. CHOOSE: GOTO step 3 or CONTINUE
6. INPUT correct password.
7. ENSURE that the UI becomes hidden.
8. CONTINUE as in step 9 of the first test.
9. STOP

