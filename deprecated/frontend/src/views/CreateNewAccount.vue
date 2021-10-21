<template>
  <div class="main-container-wrapper">
    <div class="first-view-main-wrapper">
      <div v-if="recoveryPhrase">
        <div class="page-container__title">Create New Account</div>
        <label class="reveal-recoveryPhrase__label">Your recovery phrase</label>
        <div class="recovery-phrase-container">
          <div class="recovery-phrase-container__text-container">
            <div class="recovery-phrase-container__text notranslate">
              {{ recoveryPhrase }}
            </div>
          </div>
        </div>
        <el-button
          class="main-button"
          type="primary"
          @click="handleConfirmRecoveryPhrase()"
          :disabled="passwordForm.password.length === 0"
        >
          Okay
        </el-button>
      </div>
      <el-form
        v-if="!recoveryPhrase"
        :rules="rules"
        :model="passwordForm"
        ref="passwordForm"
      >
        <a class="back-button" href="#" @click="goBack">&lt; Back</a>
        <div class="page-container__title">Create New Account</div>
        <el-form-item label="Choose a password" prop="password">
          <el-input
            type="password"
            placeholder="password"
            id="password-box"
            v-model="passwordForm.password"
            show-password
          ></el-input>
        </el-form-item>
        <el-button
          class="main-button"
          type="primary"
          @click="handleSetPassword()"
          >Set password</el-button
        >
      </el-form>
    </div>
  </div>
</template>

<script>
import backend from '@/backend';

export default {
  name: 'CreateNewAccount',
  data() {
    return {
      recoveryPhrase: '',
      passwordForm: {
        password: '',
      },
      rules: {
        password: [
          {
            required: true,
            message: 'Please choose a password',
            trigger: 'blur',
          },
          { min: 8, message: '8 characters minimum', trigger: 'blur' },
        ],
      },
    };
  },
  methods: {
    goBack() {
      this.$router.push('/create_account');
    },
    async handleSetPassword() {
      this.$refs['passwordForm'].validate(async valid => {
        if (valid) {
          this.recoveryPhrase = await backend.main.Service.CreateAccount(
            this.passwordForm.password
          );
        }
      });
    },
    handleConfirmRecoveryPhrase() {
      this.$router.push('/sign_in');
    },
  },
};
</script>
