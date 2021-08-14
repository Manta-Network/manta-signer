<template>
  <div class="unlock-page__container">
    <div class="unlock-page">
      <div class="unlock-page__logo-container">
        <div style="z-index: 0">
          <img src="../assets/logo.png" width="120px" height="120px">
        </div>
      </div>
      <h1 class="unlock-page__title">欢迎回来！</h1>
      <div>欢迎来到去中心化网络</div>
    </div>
  </div>
</template>

<script>
export default {
  name: "HomePage",
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
</style>