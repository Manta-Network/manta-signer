<template>
  <div class="main-container-wrapper">
    <div class="first-view-main-wrapper">
      <div class="first-view-main">
        <el-form
          class="import-account"
          :model="ruleForm"
          :rules="rules"
          ref="ruleForm"
        >
          <a class="back-button" href="#" @click="goBack">&lt; Back</a>
          <div class="page-container__title">
            Recover Account
          </div>
          <el-form-item label="Recovery phrase" prop="recoveryPhrase">
            <el-input
              type="textarea"
              v-model="ruleForm.recoveryPhrase"
              v-on:input="onChangeRecoveryPhraseInput()"
            ></el-input>
          </el-form-item>
          <el-form-item label="password" prop="password">
            <el-input
              type="password"
              v-model="ruleForm.password"
              autocomplete="off"
              show-password
            ></el-input>
          </el-form-item>
          <el-form-item label="confirm password" prop="passwordConfirm">
            <el-input
              type="password"
              v-model="ruleForm.passwordConfirm"
              autocomplete="off"
              show-password
            ></el-input>
          </el-form-item>
          <el-form-item>
            <el-button
              class="main-button"
              type="primary"
              @click="onSubmit('ruleForm')"
              >Recover</el-button
            >
          </el-form-item>
        </el-form>
      </div>
    </div>
  </div>
</template>

<script>
import backend from '@/backend';
import { mnemonicToLegacySeed } from '@polkadot/util-crypto';

export default {
  name: 'Recover Account',
  data() {
    let confirmPwd = (rule, value, callback) => {
      if (value !== this.ruleForm.password) {
        callback(new Error("Passwords didn't match"));
      } else {
        callback();
      }
    };
    let validateRecoveryPhrase = (rule, value, callback) => {
      try {
        mnemonicToLegacySeed(value.replace(/\s+/g, ' ').trim());
      } catch (error) {
        callback(new Error('Recovery phrase invalid'));
      }
      callback();
    };
    return {
      ruleForm: {
        password: '',
        passwordConfirm: '',
        recoveryPhrase: '',
      },
      rules: {
        password: [
          { required: true, message: 'Please enter password', trigger: 'blur' },
          {
            min: 8,
            message: 'Password must be 8 characters minimum',
            trigger: 'blur',
          },
        ],
        passwordConfirm: [
          {
            required: true,
            message: 'Please confirm password',
            trigger: 'blur',
          },
          {
            min: 8,
            message: 'Password must be 8 characters minimum',
            trigger: 'blur',
          },
          { validator: confirmPwd, trigger: 'blur' },
        ],
        recoveryPhrase: [
          {
            required: true,
            message: 'Please enter recovery phrase',
            trigger: 'blur',
          },
          { validator: validateRecoveryPhrase, trigger: 'blur' },
        ],
      },
    };
  },
  methods: {
    onChangeRecoveryPhraseInput() {
      this.signerError = null;
    },
    onSubmit(formName) {
      this.$refs[formName].validate(valid => {
        if (valid) {
          backend.main.Service.SaveRecoveredAccount(
            this.ruleForm.password,
            this.ruleForm.recoveryPhrase.replace(/\s+/g, ' ').trim()
          )
            .then(res => {
              if (res === 0) {
                this.$router.push('/sign_in');
              } else if (res === 1) {
                // Should never happen, since we check validity before submitting
                this.$message({
                  showClose: true,
                  message: 'Recovery phrase invalid',
                  type: 'error',
                });
              } else if (res === 2) {
                this.$message({
                  showClose: true,
                  message: 'Failed to save recovery phrase',
                  type: 'error',
                });
              }
            })
            .catch(() => {
              this.$message({
                showClose: true,
                message: 'Error recovering account',
                type: 'error',
              });
            });
        } else {
          return false;
        }
      });
    },
    goBack() {
      this.$router.push('/create_account');
    },
  },
};
</script>
