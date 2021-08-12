<template>
  <div class="main-container-wrapper">
    <div class="first-view-main-wrapper">
      <div class="first-view-main">
        <el-form class="import-account">
          <a class="import-account__back-button" href="#" @click="goBack">&lt; 返回</a>
          <div class="import-account__title">使用账户助记词恢复您的账户</div>
          <div class="import-account__selector-label">请输入助记词来恢复您的密码</div>

          <el-form-item label="钱包助记词">
            <template v-if="showSeed">
              <el-input type="textarea" placeholder="用空格分隔每个单词" v-model="seed"></el-input>
            </template>
            <template v-else>
              <el-input type="password" placeholder="从剪贴板粘贴账户助记词" v-model="seed"></el-input>
            </template>

          </el-form-item>
          <el-form-item label="显示账户助记词" prop="delivery">
            <el-switch v-model="showSeed"></el-switch>
          </el-form-item>

          <el-form-item label="密码" prop="pass">
            <el-input type="password" v-model="password" autocomplete="off"></el-input>
          </el-form-item>
          <el-form-item label="确认密码" prop="checkPass">
            <el-input type="password" v-model="passwordConfirm" autocomplete="off"></el-input>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="onSubmit">恢复</el-button>
          </el-form-item>
        </el-form>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "RestoreVault",
  data() {
    return {
      showSeed: false,
      password: "",
      passwordConfirm: "",
      seed: ""
    }
  },
  methods: {
    onSubmit() {
      console.log(1);
    },
    goBack() {
      window.history.length > 1 ? this.$router.go(-1) : this.$router.push('/')
    }
  }
}
</script>

<style scoped>
.main-container-wrapper {
  display: flex;
  justify-content: center;
  flex: 1 0 auto;
  min-height: 0;
  width: 100%;
}
.first-view-main-wrapper {
  display: flex;
  width: 100%;
  height: 100%;
  justify-content: center;
  padding: 0 10px;
  background: white;
}
.first-view-main-wrapper>.first-view-main {
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
}
.import-account {
  display: flex;
  flex-flow: column nowrap;
  margin: 60px 0 30px 0;
  position: relative;
  max-width: initial;
}
.import-account__back-button {
  font-size: 1rem;
  line-height: 140%;
  font-style: normal;
  font-weight: normal;
  margin-bottom: 18px;
  color: #22232c;
  position: absolute;
  top: -25px;
}
.import-account__title {
  font-size: 2.5rem;
  line-height: 140%;
  font-style: normal;
  font-weight: normal;
  color: #1b344d;
  margin-bottom: 10px;
}
.import-account__selector-label {
  font-size: 1rem;
  line-height: 140%;
  font-style: normal;
  font-weight: normal;
  color: #1b344d;
}
</style>