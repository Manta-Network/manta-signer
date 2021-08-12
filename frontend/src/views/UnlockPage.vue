<template>
  <div class="unlock-page__container">
    <div class="unlock-page">
      <div class="unlock-page__logo-container">
        <div style="z-index: 0">
          <img src="../assets/logo.png" width="120px" height="120px">
        </div>
      </div>
      <h1 class="unlock-page__title">欢迎回来！</h1>
      <div>即将进入去中心化网络</div>
      <form class="unlock-page__form">
        <div class="unlock-page__password-container">
          <div
              class="unlock-page__password-input-container">
            <el-input placeholder="请输入密码" v-model="password" show-password></el-input>
          </div>
        </div>
        <el-button class="unlock-page__unlock-button" :disabled="password.length === 0" type="primary">解锁</el-button>
      </form>
      <div class="unlock-page__links">
        <span> 或者 <button class="unlock-page__link unlock-page__link--import" @click.prevent="toRestoreVault">使用账户助记词恢复您的账户</button>
        </span>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "UnlockPage",
  data() {
    return {
      password: ""
    }
  },
  methods: {
    toRestoreVault() {
      this.$router.push('/restore_vault')
    }
  }
}
</script>

<style scoped>
.unlock-page__container {
  background-color: white;
  display: flex;
  align-self: stretch;
  justify-content: center;
  flex: 1 0 auto;
}
.unlock-page {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 357px;
  padding: 30px;
  font-weight: 400;
  color: #aeaeae;
}
.unlock-page__logo-container {
  margin-top: 24px;
}
.unlock-page__title {
  font-size: 2rem;
  line-height: 140%;
  margin-top: 5px;
  font-weight: 800;
  color: #4d4d4d;
}
.unlock-page__form {
  width: 100%;
  margin: 56px 0 8px;
}
.unlock-page__password-container {
  width: 100%;
  border: 0;
  margin: 0;
  display: inline-flex;
  padding: 0;
  position: relative;
  min-width: 0;
  flex-direction: column;
  vertical-align: top;
}
.unlock-page__password-input-container {
  margin-top: 16px;
  position: relative;
  width: 100%;
  color: rgba(0, 0, 0, 0.87);
  cursor: text;
  display: inline-flex;
  font-size: 1rem;
  box-sizing: border-box;
  align-items: center;
  font-weight: 400;
  line-height: 1.1876em;
  letter-spacing: 0.00938em;
}
.unlock-page__unlock-button {
  margin-top: 20px;
  height: 60px;
  font-weight: 400;
  box-shadow: none;
  border-radius: 4px;
  width: 100%;
}
.unlock-page__links {
  margin-top: 15px;
  width: 100%;
  text-align: center;
  font-size: 0.75rem;
}
.unlock-page__link {
  cursor: pointer;
  background-color: unset;
  color: #037dd6;
  font-size: unset;
  font-weight: bold;
  border-style: none;
}
</style>