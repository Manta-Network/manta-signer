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
              show-password
            ></el-input>
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

Events.On('manta.browser.openUnlock', function(data) {
  console.log(data + '!!!!!');
});

export default {
  name: 'UnlockPage',
  data() {
    return {
      txType: 'Transfer',
      totalAmount: 100,
      denomination: 'DOT',
      recipientAddress: '12c6DSiU4Rq3P4ZxziKxzrL5LmMBrzjrJX',
      password: '',
    };
  },
  methods: {
    async handleClickApproveTransaction() {
      console.log('unlock');
      const success = await backend.main.Service.LoadRootSeed(this.password);
      console.log(success);
      if (success) {
        Events.Emit('manta.server.onUnlockSuccess');
        this.password = '';
        this.closeWindow();
      }
      //   .then(() => {
      //     Events.Emit('manta.server.onUnlockSuccess');
      //   })
      //   .catch(err => {
      //     this.$message.error(err);
      //   });
    },
    handleClickDeclineTransaction() {
      Events.Emit('manta.server.onUnlockFail');
      this.closeWindow();
    },
  },
};
</script>
