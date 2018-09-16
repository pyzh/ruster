<template>
    <div id="help">
        <main>
            <div id="content">
                <div id="support">
                    <div id="ruster">
                        <span style="font-size: 1.6rem;">Ruster : Rust</span>
                        <span style="font-size: 1.2rem;">为中心的全栈技术社区！</span>
                    </div><br>
                    <h3>支持Ruster</h3>
                    <p>Ruster 是一个社区项目，<a href="https://github.com/OUISRC/kary" target="_blank">源码</a>&nbsp;每月需要费用花在云服务器托管，请帮助这个项目给予捐助，谢谢！
                    联系krircc@aliyun.com.</p>
                </div><br>
                <div style="box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);width:11.4rem;text-align:center;">
                    <div><img id="pay" src="../../../static/imgs/weixinpay.jpeg"></div>
                    <strong>微信支付</strong>
                </div><br>
                <div style="box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);width:11.4rem;text-align:center;">
                    <div><img id="pay" src="../../../static/imgs/alipay.jpeg"></div>
                    <strong>支付宝</strong>
                </div><br>
                <div id="info">
                    <h3>捐助人组织和个人</h3>
                    <p>尹延律&emsp;100 &emsp; 2018-07</p>
                    <p>张立群&emsp;66.66 &emsp; 2018-07</p>
                </div>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
export default {
    name: 'help'
}
</script>

<style scoped>
a {
  color: #0541af;
}
main #ruster {
    padding-top: 1rem;
    color: #b93bf3;
    font-weight: bold;
}
main img {
    widows: 11rem;
    height: 11rem;
}
@media only screen and (max-width: 600px) {
    main  {
      margin: 0 auto;
      width: 90%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
}
@media only screen and (min-width: 850px) {
    main {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
}
</style>
