<template>
  <div class="main-container-wrapper">
    <div class="first-view-main-wrapper">
      <el-form>
        <div class="page-container__title">Transaction</div>
        <p class="wrap-text">{{ txSummary }}</p>
        <el-form-item prop="password">
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
          type="main-button"
          @click="handleClickApproveTransaction"
          >Approve</el-button
        >
        <el-button
          class="decline-button"
          type="main-button"
          @click="handleClickDeclineTransaction"
          >Decline</el-button
        >
      </el-form>
    </div>
  </div>
</template>

<script>
import backend from '@/backend';
import { Events } from '@wails/runtime';

export default {
  name: 'AuthorizeTransaction',
  data() {
    return {
      txSummary: null,
      password: '',
      passwordIsInvalid: false,
    };
  },
  beforeMount() {
    this.txSummary = `${this.$route.params.txType} ${this.$route.params.amount} ${this.$route.params.currencySymbol} to ${this.$route.params.recipient}? `;
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
        this.$router.push('/');
      } else {
        this.passwordIsInvalid = true;
      }
    },
    handleClickDeclineTransaction() {
      Events.Emit('manta.server.onUnlockFail');
      backend.main.Service.WindowHide();
      this.$router.push('/');
    },
  },
};
</script>
