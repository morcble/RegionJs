var LoginPlugin  = {
		PC_lOGIN_URL:"index.html",
		PC_LANDINGPAGE_URL:"landing.html",
		PC_ACTIVE_ACCOUNT_URL:"admin/login/account_active.html",
		
		APP_lOGIN_URL:"app/login.html",
		APP_LANDINGPAGE_URL:"app/app.html",
		APP_ACTIVE_ACCOUNT_URL:"activeaccount",//screen
		preLoginFlag :"preLogin",
		/**
		 * session expired or invalid access will trigger prelogin
		 */
		preLogin:function(){
			if(RegionUtil.getCookie(LoginPlugin.preLoginFlag)!="y"){
    			RegionUtil.setCookieWithExpire(LoginPlugin.preLoginFlag,"y","s5");
    			
    			if(RegionUtil.isPc()){
    				var promise = RegionUtil.alert("会话已过期,请重新登录");
    				promise.done(function(){
    					RegionUtil.getTopWindow(window).location.replace(Config.frontendPath+LoginPlugin.PC_lOGIN_URL);
    				});	
    			}
    			else{
    				RegionUtil.getTopWindow(window).location.replace(Config.frontendPath+LoginPlugin.APP_lOGIN_URL);	
    			}
    		}
		},
		oneTimePasswordLogin:function(oneTimePassword){
			var task = RegionUtil.ajaxJsonTask(Config.backendPath+"/RCS_Auth/oneTimePasswordLogin", "POST", {"oneTimePassword":oneTimePassword},function(serverResponse,dataPara){
				 if(!serverResponse.success){
					 RegionUtil.alert(serverResponse.msg);
				 }
				 else{
					 if(RegionUtil.isPc()){
						 RegionUtil.replacePage(Config.frontendPath+LoginPlugin.PC_LANDINGPAGE_URL);
					 }
					 else{
						 RegionUtil.replacePage(Config.frontendPath+LoginPlugin.APP_LANDINGPAGE_URL);
					 }
				 }
			}, true);
			return task;
		},
		login:function(inputAccount,inputPassword,spName,verifycode){
			//var password = RegionUtil.SHA256(SparkMD5.hash(inputPassword.trim()).toUpperCase()).toUpperCase();
			var password = SparkMD5.hash(inputPassword.trim()).toUpperCase();
			var account = inputAccount.trim();
			var task = RegionUtil.ajaxJsonTask(Config.backendPath+"/RCS_Auth/login", "POST", {"account":account,"password":password,"spName":spName,"verifycode":verifycode},function(loginResultJson,dataPara){
				if(!loginResultJson.validAccount){
					if(loginResultJson.loginResponseType=="EXPIRED_VERIFYIMG"){
						RegionUtil.toast("验证码已过期，请刷新");
					}
					else if(loginResultJson.loginResponseType=="INVALID_VERIFYIMG"){
						RegionUtil.toast("验证码错误");
					}
					else if(loginResultJson.loginResponseType=="ACCOUNT_TO_BE_ACTIVED"){
						
						if(RegionUtil.isPc()){
							RegionUtil.alert("请先激活帐号");
							//window.open(Config.frontendPath+LoginPlugin.PC_ACTIVE_ACCOUNT_URL,"_self");
							//RegionUtil.toast("此账户未激活");
						}
						else{
							RegionUtil.toast("此账户未激活");
							//window.screenRegion.loadScreen(LoginPlugin.APP_ACTIVE_ACCOUNT_URL,null);
						}
					}else if (loginResultJson.loginResponseType=="ACCOUNT_LOCKED"){
						RegionUtil.toast("账号已锁定");
					}
					else{
						if(RegionUtil.isPc()){
							RegionUtil.toast("账号不存在或密码错误");
						}
						else{
							RegionUtil.toast("账号不存在或密码错误");
						}
					} 
				 }
				 else{
					 if(RegionUtil.isPc()){
						 RegionUtil.replacePage(Config.frontendPath+LoginPlugin.PC_LANDINGPAGE_URL);
					 }
					 else{
						 RegionUtil.replacePage(Config.frontendPath+LoginPlugin.APP_LANDINGPAGE_URL);
					 }
				 }
			}, true);
			return task;
		},
		logout:function(){
			return RegionUtil.ajaxJsonTask(Config.backendPath+"/RCS_Auth/logout", "POST", null,function(response,dataPara){
				 RegionUtil.delCookie("_region_login");
				 if(RegionUtil.isPc()){
						RegionUtil.getTopWindow(window).location.replace(Config.frontendPath+LoginPlugin.PC_lOGIN_URL);
	    		 }
	    		 else{
	    				RegionUtil.getTopWindow(window).location.replace(Config.frontendPath+LoginPlugin.APP_lOGIN_URL);
	    		 }
			}, true,function(){
				RegionUtil.delCookie("_region_login");
				if(RegionUtil.isPc()){
					RegionUtil.getTopWindow(window).location.replace(Config.frontendPath+LoginPlugin.PC_lOGIN_URL);
    			}
    			else{
    				RegionUtil.getTopWindow(window).location.replace(Config.frontendPath+LoginPlugin.APP_lOGIN_URL);
    			}
			});
		},
		register:function(inputAccount,inputPassword,spName,smsCode,smsCodeKey){
			var password = RegionUtil.SHA256(SparkMD5.hash(inputPassword.trim()).toUpperCase()).toUpperCase();
			var account = inputAccount.trim();

			var task = RegionUtil.ajaxJsonTask(Config.backendPath+"/RCS_Auth/register", "POST", {"account":account,"password":password,"spName":spName,"smsCode":smsCode,"smsCodeKey":smsCodeKey},function(serverResponse,dataPara){
				if(serverResponse.success){
					var promise = RegionUtil.alert("注册成功");
					promise.done(function(){
						if(RegionUtil.isPc()){
							window.open(Config.frontendPath+LoginPlugin.PC_LANDINGPAGE_URL,"_blank");
						 }
						 else{
							 RegionUtil.replacePage(Config.frontendPath+LoginPlugin.APP_LANDINGPAGE_URL);
						 }
					}); 
				}
				else{
					if(loginResultJson.respCode=="_701"){
						RegionUtil.toast("验证码错误");
					}
					else if(loginResultJson.respCode=="_702"){
						RegionUtil.toast("验证码已过期");
					}
				}
			}, true);
			return task;
		},
		activeByPassword:function(password){//用输入新密码的方式激活帐号
			var taskPromise = RegionUtil.ajaxJsonTask(Config.backendPath+"/RCS_Auth/active-account-bypasswords", "POST", {password:password},function(serverResponse,dataPara){
					if(serverResponse.loginResponseType=="LOGIN_SUCCESSFULLY"){
						var alertPromise = null;
						var msg = "帐号激活成功";
						if(RegionUtil.isPc()){
							alertPromise = RegionUtil.alert(msg);
						}
						else{
							alertPromise = RegionUtil.alert(msg);
						}
						alertPromise.done(function(){
							if(RegionUtil.isPc()){
								 window.open(Config.frontendPath+LoginPlugin.PC_LANDINGPAGE_URL,"_blank");
							 }
							 else{
								 RegionUtil.replacePage(Config.frontendPath+LoginPlugin.APP_LANDINGPAGE_URL);
							 }
						})
					}
					else{
						var alertPromise = null;
						var msg = "激活失败";
						if(RegionUtil.isPc()){
							alertPromise = RegionUtil.alert(msg);
						}
						else{
							alertPromise = RegionUtil.alert(msg);
						}
						alertPromise.done(function(){
							if(RegionUtil.isPc()){
								RegionUtil.getTopWindow(window).location.replace(Config.frontendPath+LoginPlugin.PC_lOGIN_URL);
			    			}
			    			else{
			    				RegionUtil.getTopWindow(window).location.replace(Config.frontendPath+LoginPlugin.APP_lOGIN_URL);
			    			}
						})
					}
			});
			return taskPromise;
		}
}
