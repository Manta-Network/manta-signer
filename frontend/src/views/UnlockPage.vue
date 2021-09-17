<template>
  <div class="main-container-wrapper">
    <div class="page-container">
      <div class="page-container__header">
        <div class="page-container__title">Transaction</div>
      </div>
      <div class="page-container__content">
        <el-form>
          <el-form-item
            v-bind:label="
              `${txType} ${totalAmount}${this.denomination} to ${recipientAddress}?`
            "
            prop="password"
          >
            <el-input
              type="password"
              placeholder="password"
              id="password-box"
              v-model="password"
              v-on:input="onChangePasswordInput()"
              show-password
            ></el-input>
            <div v-if="passwordIsInvalid">Invalid password</div>
          </el-form-item>
          <el-button
            class="approve-button"
            type="primary"
            @click="handleClickApproveTransaction"
            >Approve</el-button
          >
          <el-button
            class="decline-button"
            type="primary"
            @click="handleClickDeclineTransaction"
            >Decline</el-button
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
  name: 'UnlockPage',
  data() {
    return {
      txType: 'Transfer',
      totalAmount: '{amount}',
      denomination: '{ticker}',
      recipientAddress: '{recipient}',
      password: '',
      passwordIsInvalid: false,
    };
  },
  methods: {
    onChangePasswordInput() {
      this.passwordIsInvalid = false;
    },
    async handleClickApproveTransaction() {
      const success = await backend.main.Service.LoadRootSeed(this.password);
      if (success) {
        Events.Emit('manta.server.onUnlockSuccess');
        this.passwordIsInvalid = false;
        this.password = '';
        backend.main.Service.WindowHide();
      } else {
        this.passwordIsInvalid = true;
      }
    },
    handleClickDeclineTransaction() {
      Events.Emit('manta.server.onUnlockFail');
      backend.main.Service.WindowHide();
    },
  },
};
</script>
