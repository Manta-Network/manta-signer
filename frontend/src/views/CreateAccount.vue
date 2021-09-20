<template>
  <div class="main-container-wrapper">
    <div class="page-container">
      <div class="page-container__header">
        <div class="page-container__title">Create Account</div>
      </div>
      <div class="page-container__content">
        <div v-if="recoveryPhrase">
          <label class="reveal-recoveryPhrase__label"
            >Your recovery phrase</label
          >
          <div class="recovery-phrase-container">
            <div class="recovery-phrase-container__text-container">
              <div class="recovery-phrase-container__text notranslate">
                {{ recoveryPhrase }}
              </div>
            </div>
          </div>
          <el-button
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
            type="primary"
            @click="handleSetPassword()"
            :disabled="passwordForm.password.length === 0"
            >Set password</el-button
          >
        </el-form>
      </div>
    </div>
  </div>
</template>

<script>
import backend from '@/backend';

export default {
  name: 'CreateAccount',
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
          { min: 8, message: '8 digit minimal', trigger: 'blur' },
        ],
      },
    };
  },
  methods: {
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
    closeWindow() {
      backend.main.Service.WindowHide();
    },
  },
};
</script>
