<template>
  <div class="main-container-wrapper">
    <div class="page-container">
      <div class="page-container__header">
        <div class="page-container__title">Create Account</div>
      </div>
      <div class="page-container__content">
        <div class="reveal-seed__content">
          <div>
            <label class="reveal-seed__label">Your recovery phrase</label>
            <div class="export-text-container">
              <div class="export-text-container__text-container">
                <div class="export-text-container__text notranslate">
                  {{ seed }}
                </div>
              </div>
            </div>
          </div>
          <el-form :rules="rules" :model="ruleForm" ref="ruleForm">
            <el-form-item label="Choose a password" prop="password">
              <el-input
                type="password"
                placeholder="password"
                id="password-box"
                v-model="ruleForm.password"
                show-password
              ></el-input>
            </el-form-item>
            <el-button
              type="primary"
              @click="nextStep('ruleForm')"
              :disabled="ruleForm.password.length === 0"
              >Next</el-button
            >
          </el-form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import * as copy from 'copy-to-clipboard';
import backend from '@/backend';

export default {
  name: 'CreateAccount',
  data() {
    return {
      ruleForm: {
        password: '',
      },
      rules: {
        password: [
          { required: true, message: 'Please enter password', trigger: 'blur' },
          { min: 8, message: '8 digit minimal', trigger: 'blur' },
        ],
      },
      seed: '',
    };
  },
  methods: {
    copyToClipboard() {
      if (copy(this.seed)) {
        this.$message.success('Copy succeed');
      }
    },
    nextStep(formName) {
      this.$refs[formName].validate(valid => {
        if (valid) {
          backend.main.Service.AcquireSeedByPassword(this.password).then(
            seed => {
              this.seed = seed;
            }
          );
        } else {
          return false;
        }
      });
    },
    closeWindow() {
      backend.main.Service.WindowHide();
    },
    saveToCSV() {
      backend.main.Service.SaveCSV(this.seed)
        .then(() => {
          this.$message.success('Copy succeed');
        })
        .catch(err => {
          this.$message.error(err);
        });
    },
  },
};
</script>

<style scoped>
.main-container-wrapper {
  display: flex;
  justify-content: center;
  flex: 1 0 auto;
  min-height: 0;
  width: 100%;
}

.page-container {
  /* max-height: 82vh;
  min-height: 570px; */
  flex: 0 0 auto;
  margin-right: auto;
  margin-left: auto;
  width: 408px;
  background-color: #fff;
  box-shadow: 0 0 7px 0 rgb(0 0 0 / 8%);
  z-index: 25;
  display: flex;
  flex-flow: column;
  border-radius: 8px;
  overflow-y: auto;
}

.page-container__header {
  display: flex;
  flex-flow: column;
  border-bottom: 1px solid #d2d8dd;
  padding: 16px;
  flex: 0 0 auto;
  position: relative;
}

.page-container__title {
  font-size: 2rem;
  font-family: Euclid, Roboto, Helvetica, Arial, sans-serif;
  line-height: 140%;
  font-style: normal;
  font-weight: normal;
  color: #000;
  font-weight: 500;
  margin-right: 1.5rem;
}

.page-container__subtitle {
  font-size: 0.875rem;
  font-family: Euclid, Roboto, Helvetica, Arial, sans-serif;
  line-height: 140%;
  font-style: normal;
  font-weight: normal;
  padding-top: 0.5rem;
  color: #808080;
}

.page-container__content {
  overflow-y: auto;
  flex: 1;
}

.page-container__warning-container {
  background: #fdf4f4;
  padding: 20px;
  display: flex;
  align-items: flex-start;
}

.reveal-seed__content {
  padding: 20px;
}

.input-label {
  padding-bottom: 10px;
  font-weight: 400;
  display: inline-block;
}

.reveal-seed__label {
  padding-bottom: 10px;
  font-weight: 400;
  display: inline-block;
}

.export-text-container {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  border: 1px solid #dedede;
  border-radius: 4px;
  font-weight: 400;
}
.export-text-container__text-container {
  width: 100%;
  display: flex;
  justify-content: center;
  padding: 20px 0 20px 0;
  border-radius: 4px;
  background: #fafafa;
}
.export-text-container__text {
  font-size: 1.125rem;
  font-family: Euclid, Roboto, Helvetica, Arial, sans-serif;
  line-height: 140%;
  font-style: normal;
  font-weight: normal;
  resize: none;
  border: none;
  background: #fafafa;
  text-align: center;
}
.export-text-container__buttons-container {
  display: flex;
  flex-direction: row;
  border-top: 1px solid #dedede;
  width: 100%;
}
.export-text-container__button {
  font-size: 0.75rem;
  font-family: Euclid, Roboto, Helvetica, Arial, sans-serif;
  line-height: 140%;
  font-style: normal;
  font-weight: normal;
  padding: 10px;
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  color: #037dd6;
}
.export-text-container__button--copy {
  border-right: 1px solid #dedede;
}
.export-text-container__button-text {
  padding-left: 10px;
}
</style>
