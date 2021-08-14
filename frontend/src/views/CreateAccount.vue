<template>
  <div class="main-container-wrapper">
    <div class="page-container">
      <div class="page-container__header">
        <div class="page-container__title">账户助记词</div>
        <div class="page-container__subtitle">如果您更换浏览器或计算机，则需要使用此账户助记词访问您的帐户。请将它们保存在安全秘密的地方。</div>
      </div>
      <div class="page-container__content">
        <div class="page-container__warning-container">
          <svg class="page-container__warning-icon" height="32" width="33" xmlns="http://www.w3.org/2000/svg">
            <g fill="none" fill-rule="evenodd">
              <path d="M19.132 2.854l12.44 22.748a3 3 0 01-2.632 4.44H4.06a3 3 0 01-2.632-4.44l12.44-22.748a3 3 0 015.264 0z" stroke="#ff001f" stroke-width="2"/>
              <g fill="#ff001f">
                <path d="M15 8h3v13h-3zM15 23h3v3h-3z"/>
              </g>
            </g>
          </svg>
          <div class="page-container__warning-message">
            <div class="page-container__warning-title">不要对任何人展示此账户助记词！</div>
            <div>该账户助记词可以用来窃取您的所有帐户</div>
          </div>
        </div>
        <div class="reveal-seed__content">
          <template v-if="authorized">
            <div>
              <label class="reveal-seed__label">您的账户助记词</label>
              <div class="export-text-container">
                <div class="export-text-container__text-container">
                  <div class="export-text-container__text notranslate">
                    {{seed}}
                  </div>
                </div>
                <div class="export-text-container__buttons-container">
                  <div class="export-text-container__button export-text-container__button--copy" @click="copyToClipboard">
                    <svg width="17" height="17" viewBox="0 0 11 11" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path fill-rule="evenodd" clip-rule="evenodd"
                            d="M0 0H1H9V1H1V9H0V0ZM2 2H11V11H2V2ZM3 3H10V10H3V3Z" fill="#3098DC"></path>
                    </svg>
                    <div class="export-text-container__button-text">复制到剪贴板</div>
                  </div>
                  <div class="export-text-container__button" @click="saveToCSV">
                    <svg height="18" width="20" xmlns="http://www.w3.org/2000/svg">
                      <g fill="none" fill-rule="evenodd" stroke="#259de5" stroke-width="2">
                        <path d="M1.336 11v6h17v-6M9.836 0v11"/>
                        <path d="M4.524 7l5.312 4 5.313-4"/>
                      </g>
                    </svg>
                    <div class="export-text-container__button-text">保存为 CSV 文件</div>
                  </div>
                </div>
              </div>
            </div>
          </template>
          <template v-else>
            <form><label class="input-label" for="password-box">输入密码以继续</label>
              <div class="input-group">
                <el-input type="password" placeholder="密码" id="password-box" v-model="password"></el-input>
              </div>
            </form>
          </template>
        </div>
      </div>
      <div class="page-container__footer">
        <footer v-if="authorized">
          <el-button @click="closeWindow">关闭</el-button>
        </footer>
        <footer v-else>
          <el-button type="info">取消</el-button>
          <el-button type="primary" @click="nextStep" :disabled="password.length === 0">下一步</el-button>
        </footer>
      </div>
    </div>
  </div>
</template>

<script>
import * as copy from 'copy-to-clipboard'
import {ExportToCsv} from 'export-to-csv'
import backend from "@/backend";
export default {
  name: "CreateAccount",
  data() {
    return {
      password: "",
      authorized: false,
      seed: ""
    }
  },
  methods: {
    copyToClipboard() {
      if (copy(this.seed)) {
        this.$message.success("复制成功");
      }
    },
    nextStep() {
      this.authorized = true
      backend.main.Service.AcquireSeedByPassword(this.password).then((seed) => {
        this.seed = seed
      })
    },
    closeWindow() {
      this.$router.push('/')
      // backend.main.Service.WindowHide();
    },
    saveToCSV() {
      let data = [{
        seed: this.seed,
      }]
      const csvExporter = new ExportToCsv();
      csvExporter.generateCsv(data);
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

.page-container {
  max-height: 82vh;
  min-height: 570px;
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

.page-container__warning-icon {
  padding-top: 5px;
}

.page-container__warning-message {
  padding-left: 15px;
}

.page-container__warning-title {
  font-weight: 500;
}

.reveal-seed__content {
  padding: 20px;
}

.input-label {
  padding-bottom: 10px;
  font-weight: 400;
  display: inline-block;
}

.page-container__footer {
  display: flex;
  flex-flow: column;
  justify-content: center;
  border-top: 1px solid #d2d8dd;
  flex: 0 0 auto;
}

.page-container__footer > footer {
  display: flex;
  flex-flow: row;
  justify-content: center;
  padding: 16px;
  flex: 0 0 auto;
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