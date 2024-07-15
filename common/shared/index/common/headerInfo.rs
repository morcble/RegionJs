<style type="text/css">
#REGION i{
	font-size: 1.4rem;
}
#REGION .account-menu>a{
	height: 3rem;
}
</style>

<div id="REGION" class="hidden">
		<ul class="flex-content">
                <div class="user-hover">
                    <li class="user-admin">
                       <i class="fa-solid fa-user"></i>
                    </li>
                    <div class="account-menu">
                        <a class=" text-overflow ">
                            <i class="fa-solid fa-user-circle"></i>
                            <span class="userName"></span>
                        </a>
                        <a onclick="REGION.updatePwd(event)">
                            <i class="fa-solid fa-unlock-alt"></i>
                            <span>修改密码</span>
                        </a>
                        <a onclick="REGION.configTheme(event)">
                            <i class="fa-solid fa-palette"></i>
                            <span>主题</span>
                        </a>
                        <a onclick="REGION.goLanding(event)">
                            <i class="fa-solid fa-house-return"></i>
                            <span>返回</span>
                        </a>
                        <a  onclick="REGION.logout(event)">
                            <i class="fa-solid fa-sign-out fa-lg"  aria-hidden="true"></i></li>
                            <span>退出</span>
                        </a>
                    </div>
                </div>
<!-- 				 <li onclick="REGION.logout(event)" style="width:0.625rem;text-align: right"><i class="fa fa-sign-out fa-lg"  aria-hidden="true"></i></li> -->
            </ul>
            
            <div class="draw"></div>
</div>	


<script type="text/javascript">
var REGION = {
		goLanding:function(){
			location.replace("/sp/rs/landing.html","_self");
		},
        // showNumber:0,
        beforeRenderData:function(){
        },
        updatePwd:function(event){
        	RegionUtil.openModalWindow("common/shared/index/common/updatePassWord.rs","修改密码",1,null,null,500,300);
        },
        logout:function(event){
            RegionUtil.confirm("确认要登出系统?","提示",function(){
                LoginPlugin.logout();
            });
        },
        //侧边弹出设置
        configTheme:function (event){
            openDrawerWindow(RS.appendParaMapToUrl("common/shared/theme/theme.rs",null),"主题配置","50%");
        },
        changeLocale:function(elem,event){
            var locale = RegionUtil.getCurrentLocale();
            if(locale==""||locale=="cn"){
                RegionUtil.setLocale("en");
                $(elem).find(".locale-btn").text("中文");
            }
            else{
                RegionUtil.setLocale("cn");
                $(elem).find(".locale-btn").text("English");
            }
        },
        afterRenderData:function(){
            let curRegion = this;
            curRegion.find(".userName").hover(function () {
                curRegion.find(".account-menu").fadeIn()
            })
            var locale = RegionUtil.getCurrentLocale();
            if(locale==""||locale=="cn"){
            	curRegion.find(".locale-btn").text("English");
            }
            else{
            	curRegion.find(".locale-btn").text("中文");
            }
            var loginInfo = RegionUtil.getCookie("_region_login");
            if(loginInfo!=null){
                try{
                    loginInfo = JSON.parse(loginInfo);
                    this.find(".userName").text(loginInfo.userName);
                }
                catch(e){
                    console.log(e)
                }
            }
        }
    };


RegionUtil.ready(function(){
	var region = RegionUtil.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.beforeRenderData = REGION.beforeRenderData;
	region.renderRegion();
})
</script>