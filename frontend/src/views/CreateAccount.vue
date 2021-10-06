<template>
  <div class="main-container-wrapper">
    <div class="page-container">
      <div class="page-container__header">
        <div class="page-container__title">Create Account</div>
      </div>
      <div class="page-container__content">
        <el-form>
          <el-button
            class="approve-button"
            type="primary"
            @click="handleClickCreateNewAccount"
            >New Account</el-button
          >
          <el-button
            class="decline-button"
            type="primary"
            @click="handleClickRecoverExistingAccount"
            >Recover Account</el-button
          >
        </el-form>
      </div>
    </div>
  </div>
</template>

<script>
import backend from '@/backend';
import { Events } from '@wails/runtime';

export default {
  name: 'CreateAccount',
  data() {
    return {};
  },
  methods: {
    handleClickCreateNewAccount() {
      this.$router.push('/create_new_account');
    },
    handleClickRecoverExistingAccount() {
      this.$router.push('/recover_account');
    },
    handleClickDeclineTransaction() {
      Events.Emit('manta.server.onUnlockFail');
      backend.main.Service.WindowHide();
    },
  },
};
</script>
