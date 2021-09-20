<template>
  <div class="main-container-wrapper">
    <div class="page-container">
      <div class="page-container__header">
        <div class="page-container__title">Sign In</div>
      </div>
      <div class="page-container__content">
        <el-form>
          <el-form-item prop="password">
            <el-input
              type="password"
              placeholder="password"
              id="password-box"
              v-model="password"
              show-password
            ></el-input>
          </el-form-item>
          <el-button type="primary" @click="handleClickSignIn"
            >Sign In</el-button
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
  name: 'SignInPage',
  data() {
    return {
      passwordIsInvalid: false,
      password: '',
    };
  },
  methods: {
    async handleClickSignIn() {
      const success = await backend.main.Service.LoadRootSeed(this.password);
      if (success) {
        Events.Emit('manta.server.onUnlockSuccess');
        this.password = '';
        backend.main.Service.WindowHide();
      } else {
        this.passwordIsInvalid = true;
      }
    },
  },
};
</script>
