<template>
  <div class="main-container-wrapper">
    <div class="first-view-main-wrapper">
      <div class="first-view-main">
        <el-form>
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
            class="main-button"
            type="primary"
            @click="handleClickSignIn"
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
  name: 'SignIn',
  data() {
    return {
      password: '',
      passwordIsInvalid: false,
    };
  },
  methods: {
    onChangePasswordInput() {
      this.passwordIsInvalid = false;
    },
    async handleClickSignIn() {
      const success = await backend.main.Service.LoadRootSeed(this.password);
      if (success) {
        Events.Emit('manta.server.onUnlockSuccess');
        this.password = '';
        backend.main.Service.WindowHide();
        this.$router.push('/');
      } else {
        this.passwordIsInvalid = true;
      }
    },
  },
};
</script>
