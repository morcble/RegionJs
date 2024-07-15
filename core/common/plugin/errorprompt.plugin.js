var ErrorPromptPlugin = {
		showErrorMsg:function(targetObj,fieldError,region){
			//var regionAttr = targetObj.getAttribute("region-attr");
			var targetRegion = RegionUtil.getRegionByElem(targetObj);
			ErrorPromptPlugin.cleanErrorMsg(targetObj,fieldError,targetRegion);
			
			if(targetObj.getAttribute("type")=="hidden"||$(targetObj).hasClass("wrapped")){
				targetObj = $(targetObj).parent()[0];//针对wrapper的验证 ,如select wrapper
			}
			var msgHtml = '<div class="error-msg">'+fieldError.errorMsg+'</div>';
			RegionUtil.insertAfter(targetRegion,targetObj,msgHtml);
		},
		cleanErrorMsg:function(targetObj,fieldError,region){
			if(targetObj.getAttribute("type")=="hidden"||$(targetObj).hasClass("wrapped")){
				targetObj = $(targetObj).parent()[0];//针对wrapper的验证 ,如select wrapper
			}
			var nextObj = $(targetObj).next();
			if(nextObj.hasClass("error-msg"))nextObj.remove();
		},
		cleanErrorsForRegion:function(region){//释放资源
			region.find(".error-msg").remove();
			//$(region).parent().removeClass('is-error');
		}
}