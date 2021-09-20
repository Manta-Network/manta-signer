<template>
  <div class="main-container-wrapper">
    <div class="first-view-main-wrapper">
      <div class="first-view-main">
        <el-form class="import-account" :model="ruleForm" :rules="rules" ref="ruleForm">
          <a class="import-account__back-button" href="#" @click="goBack">&lt; Back</a>
          <div class="import-account__title">Manta Mnemonics can be used to recover your account.</div>
          <div class="import-account__selector-label">Manta Mnemonics can be used to recover your password.</div>

          <el-form-item label="Manta Mnemonics" prop="seed">
            <template v-if="ruleForm.showSeed">
              <el-input type="textarea" placeholder="Use space to split words" v-model="ruleForm.seed" ></el-input>
            </template>
            <template v-else>
              <el-input type="password" placeholder="copy to clipboard" v-model="ruleForm.seed"></el-input>
            </template>
          </el-form-item>

          <el-form-item label="show mnemonics" prop="delivery">
            <el-switch v-model="ruleForm.showSeed"></el-switch>
          </el-form-item>

          <el-form-item label="password" prop="password">
            <el-input type="password" v-model="ruleForm.password" autocomplete="off" show-password></el-input>
          </el-form-item>
          <el-form-item label="confirm password" prop="passwordConfirm">
            <el-input type="password" v-model="ruleForm.passwordConfirm" autocomplete="off" show-password></el-input>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="onSubmit('ruleForm')">recover</el-button>
          </el-form-item>
        </el-form>
      </div>
    </div>
  </div>
</template>

<script>
import backend from "@/backend";

export default {
  name: "RestoreVault",
  data() {
    let confirmPwd = (rule, value, callback) => {
      if (value !== this.ruleForm.password) {
        callback(new Error("Password didn't match"))
      } else {
        callback()
      }
    }
    return {
      ruleForm: {
        showSeed: false,
        password: '',
        passwordConfirm: '',
        seed: ''
      },
      rules: {
        password: [
          { required: true, message: 'please enter password', trigger: 'blur' },
          { min: 8, message: 'characters should be 8 length', trigger: 'blur' }
        ],
        passwordConfirm: [
          { required: true, message: 'please enter password', trigger: 'blur' },
          { min: 8, message: 'characters should be 8 length', trigger: 'blur' },
          { validator: confirmPwd, trigger: 'blur'}
        ],
        seed: [
          { required: true, message: 'please enter mnemonics', trigger: 'blur' }
        ],
      }
    }
  },
  methods: {
    onSubmit(formName) {
      this.$refs[formName].validate((valid) => {
        if (valid) {
          backend.main.Service.RecoverAccount(this.ruleForm.seed, this.ruleForm.password)
          .then(() => {
            backend.main.Service.WindowHide()
          }).catch(err => {
            this.$message.error(err)
          })
        } else {
          return false
        }
      })
    },
    goBack() {
        this.$router.push('/unlock')
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