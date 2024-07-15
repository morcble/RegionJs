<!-- 屏幕组件  应用最基本的构成是由多个屏幕组件组合而成-->
<style type="text/css">
#REGION{
    position: relative;
    overflow: hidden;
}
#REGION .screen{
    width: 100%;
    height: 100%;
}

</style>

<div id="REGION" class="hidden">
</div>


<script type="text/javascript">
var REGION = {
		getUrlParaMap:function(){
			var paraMap = new HashMap();
			var q = location.search.substr(1);
			var qs = q.split("&");   
			if (qs) {   
				for (var i=0;i<qs.length;i++) { 
					var  key = qs[i].substring(0,qs[i].indexOf("="));
					if(key==null||key=="") continue;
					paraMap.put(qs[i].substring(0,qs[i].indexOf("=")),decodeURIComponent(qs[i].substring(qs[i].indexOf("=")+1)));
				 }   
			} 
			return paraMap;
		},
		getTargetScreen:function(){
			var screenHolderRegion = this;
			var screenMetaArray = location.hash.split("#");
			var screenParaMap = null;
			var targetScreenName = null;
			
			if(screenHolderRegion.ignoreSearchPara) {
				screenParaMap = new HashMap();
			}
			else {
				screenParaMap = REGION.getUrlParaMap();
			}
			for(var i = 0 ; i<screenMetaArray.length;i++){
				var screenMeta = screenMetaArray[i];
				if(screenMeta=="")continue;
				var screenStr = null;
				var commaIndex = screenMeta.indexOf("?");
				
				if(commaIndex!=-1)screenStr = screenMeta.substring(0,commaIndex); //有参数
				else screenStr = screenMeta;
				
				var screenStrArr = screenStr.split(".");
				var screenHoderRegionId = screenStrArr[0];

				if(screenHoderRegionId==screenHolderRegion.regionId){//参数属于当前屏
					var parasStr = screenMeta.substring(commaIndex+1);
					var qs = parasStr.split("&");  
					if (qs) {   
						for (var j=0;j<qs.length;j++) { 
							var  key = qs[j].substring(0,qs[j].indexOf("="));
							if(key==null||key=="") continue;
							screenParaMap.put(qs[j].substring(0,qs[j].indexOf("=")),decodeURIComponent(qs[j].substring(qs[j].indexOf("=")+1)));
						 }   
					} 
					if(screenStrArr.length==2){
						targetScreenName = screenStrArr[1];
					}
					break;
				}
			}
			if(targetScreenName==null){
				targetScreenName = screenHolderRegion.screensInfo[0].name;
				screenParaMap = null;
			}
			return {"targetScreenName":targetScreenName,"screenParaMap":screenParaMap};
		},
		getScreenInfoByScreenUUID:function(screenUUID){
			return this.screenIdMap.get(screenUUID);
		},
		getScreenInfo:function(screenName){
			return this.screenMap.get(screenName);
		},
		getScreenContainer:function(){
			return $(this.getRegionDivElem());
		},
		renderContent:function(){
			var curRegion = this;
			if(!curRegion.inited){
				curRegion.inited = true;
				//TODO Url base64加密
				
				curRegion.render = REGION.render;
				
				//加载屏幕start
				curRegion.loadScreen = REGION.loadScreen;
				curRegion.unloadPrevious = REGION.unloadPrevious;//卸载上一个屏幕
				//加载屏幕end
				
				//屏幕返回start
				curRegion.unloadPreviousForScreenBack = REGION.unloadPreviousForScreenBack;//卸载上一个屏幕
				//屏幕返回end
				
				//屏幕跳转start
				curRegion.redirectScreen = REGION.redirectScreen;
				
				curRegion.getScreenInfo = REGION.getScreenInfo;
				curRegion.getScreenInfoByScreenUUID = REGION.getScreenInfoByScreenUUID;
				curRegion.pushState = REGION.pushState;
				curRegion.getScreenContainer = REGION.getScreenContainer;
				curRegion.goBack = REGION.goBack;
				curRegion.getTargetScreen = REGION.getTargetScreen;
				
				if(window.regionScreenHis==null) window.regionScreenHis= [];//history state 链表
				
				/* 
				curRegion.clearBackState = REGION.clearBackState;
				curRegion.jumpStep = null; */
			}
		},
		loadSingleScreenHolderForScreenBack:function(screenHolderRegion,singleScreenHolder){
			var curRegion = screenHolderRegion;
			
			singleScreenHolder.css("z-index",curRegion.screenDepth-1);
			curRegion.getScreenContainer().css("pointer-events","none");//禁止点击
			curRegion.unloadPreviousForScreenBack();
			singleScreenHolder.removeClass("hidden");
			singleScreenHolder.appendTo(curRegion.getScreenContainer());//事件绑定不兼容dom刷新
			curRegion.getScreenContainer().css("pointer-events","unset");
		},
		unloadPreviousForScreenBack:function(){
			var curRegion = this;
			var previousScreenId = curRegion.previousScreenId;
			if(previousScreenId==null)return;
			
			var screenInfo = curRegion.getScreenInfoByScreenUUID(previousScreenId);
			var screenName = screenInfo.name;
			//移除旧节点
			var removedNode = curRegion.getScreenContainer().children("."+previousScreenId+"_holder");
			if(curRegion.getScreenInfoByScreenUUID(previousScreenId).useCache==true){
				removedNode.find("script").remove();//避免动态添加和删除节点的时候script反复执行
				curRegion.cachedDom.put(previousScreenId,removedNode);//只缓存dom
				
				if(curRegion.exitScreenAnimation!=null){
					removedNode.bind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend', function(){
						var targetHolder = $(this);
						if(targetHolder.hasClass(curRegion.exitScreenAnimation)){
							targetHolder.removeClass(curRegion.exitScreenAnimation);
							curRegion.getScreenContainer().css("pointer-events","unset");
						}
						targetHolder.unbind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend');//取消动画监听
						window.regionScreenAnimation--;
						
						removedNode.addClass("hidden");//removedNode.remove();事件绑定不兼容dom刷新
					});
					window.regionScreenAnimation++;
					removedNode.addClass(curRegion.exitScreenAnimation);
				}
				else{
					removedNode.addClass("hidden");//removedNode.remove();事件绑定不兼容dom刷新
				}
				
			}
			else{
				var toRemoveRegion = RegionUtil.getRegionById(previousScreenId);
				curRegion.screenIdMap.remove(previousScreenId);//注销到screenIdMap
				var loadedMapForThisScreen = curRegion.loadedScreenMap.get(toRemoveRegion.screenData.screenName);
				loadedMapForThisScreen.remove(toRemoveRegion.screenData.paraJson);//注销到loadedMap
				
				if(curRegion.exitScreenAnimation!=null){
					removedNode.bind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend', function(){
						var targetHolder = $(this);
						if(targetHolder.hasClass(curRegion.exitScreenAnimation)){
							targetHolder.removeClass(curRegion.exitScreenAnimation);
							curRegion.getScreenContainer().css("pointer-events","unset");
						}
						targetHolder.unbind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend');//取消动画监听
						window.regionScreenAnimation--;
						
						delete toRemoveRegion.screenData;
						toRemoveRegion.release();//无缓存则释放
						removedNode.remove();
					});
					window.regionScreenAnimation++;
					removedNode.addClass(curRegion.exitScreenAnimation);
				}
				else{
					var targetHolder = $(this);
					delete toRemoveRegion.screenData;
					toRemoveRegion.release();//无缓存则释放
					removedNode.remove();
				}
			}
		},
		
		loadSingleScreenHolderForLoadScreen:function(screenHolderRegion,singleScreenHolder,append){
			var curRegion = screenHolderRegion;
			
			singleScreenHolder.attr("region-id",screenHolderRegion.regionId);
			singleScreenHolder.css("z-index",curRegion.screenDepth);
			curRegion.getScreenContainer().css("pointer-events","none");//禁止点击
			if(curRegion.enterScreenAnimation!=null){
				singleScreenHolder.addClass(curRegion.enterScreenAnimation);//开屏动画
				//动画事件执行完毕回调
				singleScreenHolder.bind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend', function(){
					var targetHolder = $(this);
					if(targetHolder.hasClass(curRegion.enterScreenAnimation)){
						curRegion.unloadPrevious();
						targetHolder.removeClass(curRegion.enterScreenAnimation);
						curRegion.getScreenContainer().css("pointer-events","unset");
					}
					targetHolder.unbind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend');//取消动画监听
					window.regionScreenAnimation--;
				});
				window.regionScreenAnimation++;
			}
			else{
				curRegion.unloadPrevious();
				curRegion.getScreenContainer().css("pointer-events","unset");
			}
			if(append)singleScreenHolder.appendTo(curRegion.getScreenContainer());
		    else {
		    	singleScreenHolder.removeClass("hidden");//singleScreenHolder.appendTo(curRegion.getScreenContainer());事件绑定不兼容dom刷新
		    }
		},
		loadScreen:function(screenName,paraMap,pushHistory,direction){//加载screen,pushHistory 默认为true,记录跳转了历史记录
			var loadScreenDefer = new jQuery.Deferred();
			
			if(pushHistory!=false)pushHistory=true;
			if(direction!="back")direction = "forward";
			
			var curRegion = this;
			//把参数传到screen组件开始
			var curScreenParaMap = null;
			var paraMapForCache = null;
			if(curRegion.ignoreSearchPara) {
				curScreenParaMap = new HashMap();
				paraMapForCache = new HashMap();
			}
			else {
				curScreenParaMap = REGION.getUrlParaMap();
				paraMapForCache = REGION.getUrlParaMap();
			}
			curScreenParaMap.put("screenName",screenName);
			paraMapForCache.put("screenName",screenName);
			
			if(paraMap!=null){
				var keySet = paraMap.keySet();
				for(var i = 0 ; i <keySet.length;i++){
					curScreenParaMap.put(keySet[i],paraMap.get(keySet[i]));
					paraMapForCache.put(keySet[i],paraMap.get(keySet[i]));
				}
			}
			
			
			
			if(curRegion.ignoreSearchPara==false){
				if(curRegion.ignoreParasForScreen!=null){
					for(var i = 0; i < curRegion.ignoreParasForScreen.length;i++){
						paraMapForCache.remove(curRegion.ignoreParasForScreen[i]);//如果不忽略此参数，当参数改变的时候会创建新的屏
					}
				}
			}
			

			var screenInfo = curRegion.getScreenInfo(screenName);
			if(screenInfo==null){
				RegionUtil.alert("Screen "+screenName+" does not exist:");
				return;
			}
			
			var paraJson = RegionUtil.appendParaMapToUrl("",paraMapForCache);
			console.log(paraJson)
			//如果没有加载含有相同paramap的region 则重新加载region并注册到已加载屏幕
			var loadedMapForThisScreen = curRegion.loadedScreenMap.get(screenName);
			var newScreenFlag = true;
			
			if(loadedMapForThisScreen==null){
				loadedMapForThisScreen = new HashMap();
				curRegion.loadedScreenMap.put(screenName,loadedMapForThisScreen);
			}
	
			var screenUUID = loadedMapForThisScreen.get(paraJson);
			if(screenUUID!=null){
				if(curRegion.showingScreenId==screenUUID){
					//同一个屏幕且参数一样，则不处理
					loadScreenDefer.resolve(0);
					return loadScreenDefer.promise();
				}
				curRegion.previousScreenId = curRegion.showingScreenId;
				
				var singleScreenHolder = curRegion.cachedDom.remove(screenUUID);

				if(direction=="forward")REGION.loadSingleScreenHolderForLoadScreen(curRegion,singleScreenHolder,false);
				else REGION.loadSingleScreenHolderForScreenBack(curRegion,singleScreenHolder);
				
				curRegion.showingScreenId = screenUUID;//修改screenId
				
				//record history
				if(pushHistory)curRegion.pushState(screenName,curRegion.showingScreenId);
				loadScreenDefer.resolve(1);
				
				if(typeof(curRegion.onSwitch) === "function"){
					curRegion.onSwitch(screenName,RS.getRegionById(screenUUID));
				}
				return loadScreenDefer.promise();
			}
			else{
				var screenUUID = "screen_"+RegionUtil.UUID();
				curRegion.previousScreenId = curRegion.showingScreenId;
				
				var newScreenHolder = $("<div class='screen "+screenUUID+"_holder'></div>");

				if(direction=="forward")REGION.loadSingleScreenHolderForLoadScreen(curRegion,newScreenHolder,true);
				else REGION.loadSingleScreenHolderForScreenBack(curRegion,newScreenHolder);
				
				var promise = RegionUtil.loadRegion(newScreenHolder,RegionUtil.appendParaMapToUrl(screenInfo.screenRegion,curScreenParaMap),screenUUID);
				promise.done(function(loadedRegion){
					curRegion.screenIdMap.put(screenUUID,screenInfo);//注册到screenIdMap
					loadedMapForThisScreen.put(paraJson,screenUUID);//注册到loadedMap
					
					loadedRegion.screenData = {};
					loadedRegion.screenHolderRegion = curRegion;
					loadedRegion.screenData.screenUUID = screenUUID;
					loadedRegion.screenData.screenName = screenName;
					loadedRegion.screenData.paraJson = paraJson;
					loadedRegion.screenData.screenParaMap = curScreenParaMap;
					
					curRegion.showingScreenId = screenUUID;//修改screenId
					//record history
					if(pushHistory)curRegion.pushState(screenName,curRegion.showingScreenId);
					loadScreenDefer.resolve(2);
					if(typeof(curRegion.onSwitch) === "function"){
						curRegion.onSwitch(screenName,loadedRegion);
					}
				})
				return loadScreenDefer.promise();
			}
		},
		unloadPrevious:function(){
			var curRegion = this;
			var previousScreenId = curRegion.previousScreenId;
			if(previousScreenId==null)return;
			
			var screenInfo = curRegion.getScreenInfoByScreenUUID(previousScreenId);
			var screenName = screenInfo.name;
			//移除旧节点
			var removedNode = curRegion.getScreenContainer().children("."+previousScreenId+"_holder");
			if(curRegion.getScreenInfoByScreenUUID(previousScreenId).useCache==true){
				removedNode.find("script").remove();//避免动态添加和删除节点的时候script反复执行
				curRegion.cachedDom.put(previousScreenId,removedNode);//只缓存dom
				if(screenInfo.useCache==true)
					removedNode.addClass("hidden");
				else removedNode.remove(); 
			}
			else{
				var toRemoveRegion = RegionUtil.getRegionById(previousScreenId);
				curRegion.screenIdMap.remove(previousScreenId);//注销到screenIdMap
				var loadedMapForThisScreen = curRegion.loadedScreenMap.get(toRemoveRegion.screenData.screenName);
				loadedMapForThisScreen.remove(toRemoveRegion.screenData.paraJson);//注销到loadedMap
				
				delete toRemoveRegion.screenData;
				toRemoveRegion.release();//无缓存则释放
			
				if(screenInfo.useCache==true)
					removedNode.addClass("hidden");
				else removedNode.remove(); 
			}
		},
		//多个screen嵌套 调用history.pushState()或history.replaceState()不会触发popstate事件
		//loadedRegion.render(screensInfo,false,["movein-animation4-class","moveout-animation4-class"]);
		render:function(screensInfo,ignoreSearchPara,ignoreParasForScreen,animations){
			var curRegion = this;
			curRegion.screensInfo = screensInfo;
			curRegion.ignoreParasForScreen = ignoreParasForScreen;//与跳屏无关的属性，当ignoreSearchPara=false才生效
			curRegion.showingScreenId = null;//正在展示的screenId
			curRegion.screenDepth = 0;
			curRegion.urlEncode = true;
			
			if(window.regionScreenAnimation==null)window.regionScreenAnimation = 0;//记录正在进行屏幕切换动画的数量
			curRegion.cachedDom = new HashMap();//缓存screen dom
			
			if(animations!=null){
				curRegion.enterScreenAnimation = animations[0];//TODO 参数传入  开屏效果
				curRegion.exitScreenAnimation = animations[1];;//TODO 参数传入 退屏效果
			}
			
			curRegion.screenMap = new HashMap();
			for(var i = 0 ; i<screensInfo.length ;i++){
				curRegion.screenMap.put(screensInfo[i].name,screensInfo[i]);
			}
			
			curRegion.loadedScreenMap = new HashMap();//记录已经加载的screen ,screenName -> [para ->screenUUID]
			curRegion.screenIdMap = new HashMap();//screen uuid  ->  screenInfo
			window.addEventListener("popstate",REGION.onPopstate.bind(curRegion));
			
			if(ignoreSearchPara==null)ignoreSearchPara=false;
			curRegion.ignoreSearchPara = ignoreSearchPara;//是否忽略浏览器的search参数
			//解析screenRegion
			var targetScreenData = curRegion.getTargetScreen();
			return curRegion.loadScreen(targetScreenData.targetScreenName,targetScreenData.screenParaMap,true,"forward");
		},
		pushState:function(screenName,screenUUID){
			//console.log("goto screen="+screenName)
			var curRegion = this;

			curRegion.screenDepth++;
			//console.log("screen "+curRegion.regionId +":depth"+this.screenDepth)
			if(curRegion.screenDepth>1){
				curRegion.find(".screen-back-btn").removeClass("hidden");
			}
			else{
				curRegion.find(".screen-back-btn").addClass("hidden");
			}
			
			var href = window.location.href;
			var index = href.indexOf("#");
			if(index!=-1)
				href = href.substring(0,index);
			var stateUrl = href+REGION.genStateUrl(curRegion,screenName,RegionUtil.getRegionById(screenUUID).screenData.screenParaMap);
			
			var screenObject = {};
			screenObject.id = RegionUtil.UUID();
			screenObject.screenHolderRegionId = curRegion.regionId;
			screenObject.screenName = screenName;
			screenObject.screenUUID = screenUUID;
			screenObject.depth = curRegion.screenDepth;
			screenObject.stateUrl = stateUrl;
			
			window.regionScreenHis.push(screenObject);//框架screen总体链表
			//console.log(window.regionScreenHis)
			history.pushState("", "", stateUrl);//修改浏览器链接
		},
		genStateUrl:function(screenHolderRegion,curScreenName,curScreenParaMap){//获取最新的URL
			//console.log(curScreenParaMap.values())
			var urlParaMap = REGION.getUrlParaMap();
			var screenMetaArray = location.hash.split("#");
			var stateUrl = "";
			
			var screenMetaReg = {};
			for(var i = 0 ; i<screenMetaArray.length;i++){
				var screenParaMap = new HashMap();
				var screenMeta = screenMetaArray[i];
				if(screenMeta=="")continue;
				var screenStr = null;
				var commaIndex = screenMeta.indexOf("?");
				
				if(commaIndex!=-1)screenStr = screenMeta.substring(0,commaIndex); //有参数
				else screenStr = screenMeta;
				
				var screenStrArr = screenStr.split(".");
				var screenHoderRegionId = screenStrArr[0];

				if(screenHoderRegionId==screenHolderRegion.regionId){//参数属于当前屏
					screenMeta = screenHolderRegion.regionId+"."+curScreenName;
					if(curScreenParaMap!=null){
						screenMeta = RegionUtil.appendParaMapToUrl(screenMeta,curScreenParaMap);
					}
				}
				screenMetaReg[screenHoderRegionId] = screenMeta;
			}
			
			if(screenMetaReg[screenHolderRegion.regionId]==null){//如果没有当前screen的信息 则自动补充
				var screenMeta = screenHolderRegion.regionId+"."+curScreenName;
				if(curScreenParaMap!=null){
					screenMeta = RegionUtil.appendParaMapToUrl(screenMeta,curScreenParaMap);
				}
				screenMetaReg[screenHolderRegion.regionId] = screenMeta;
			}
			
			for(x in screenMetaReg){
				stateUrl=stateUrl+"#"+screenMetaReg[x];
			}
			
			return stateUrl;
		},
		
		goBack:function(step){//screen回退
			if(step==null||step>-1)step =-1;
			var curRegion = this;	
			
			var screenHisArray = window.regionScreenHis;
			if(screenHisArray[screenHisArray.length-1].screenHolderRegionId == curRegion.regionId){//如果当前屏就是最后一个state直接调用浏览器go -1
				history.go(-1);
			}
			else{
				for(var i = (screenHisArray.length-1);i >=0;i--){
					if(screenHisArray[i].screenHolderRegionId == curRegion.regionId){
						var tmp = screenHisArray[i];
						screenHisArray.splice(i,1);
						screenHisArray.push(tmp);
						break;
					}
				}
				history.go(-1);
			}
		},
		onPopstate:function(event){//需要兼容浏览器回退和screen回退
			var curRegion = this;
			if(window.regionScreenHis.length<2){//循环退出主页面
				history.go(-1);
				return;
			}
			if(window.regionScreenAnimation>0){//有屏幕正在切换,则忽略此回退
				history.pushState("", "", window.location.href);//需要推入一个state来抵消回退的state
				return;
			}
	
			var totalDepth = window.regionScreenHis.length;
			var popScreenObject = window.regionScreenHis[totalDepth-1];//需要刷新UI的的screenHolder
			if(popScreenObject.screenHolderRegionId != curRegion.regionId){
				return;//历史记录不是当前screenHolder则不处理
			}
			var targetScreenObject = window.regionScreenHis[totalDepth-2];	
			history.replaceState("", "", targetScreenObject.stateUrl);
			
			var targetScreenName = null;
			for(var i = (window.regionScreenHis.length-2);i >=0;i--){//计算要加载的屏
				if(window.regionScreenHis[i].screenHolderRegionId == curRegion.regionId){
					targetScreenName = window.regionScreenHis[i].screenName;
					break;
				}
			}
			curRegion.screenDepth--;
			if(targetScreenName!=null){
				var targetScreenData = curRegion.getTargetScreen(targetScreenName);
				curRegion.loadScreen(targetScreenName,targetScreenData.screenParaMap,false,"back");
				setTimeout(function(){
					window.regionScreenHis.pop();
				},10);
			}
			else{
				window.regionScreenHis.pop();
				history.go(-1);
			}
		},
		
		onRelease:function(){
			var curRegion = this;
			delete curRegion.screensInfo ;
			delete curRegion.showingScreenId;//正在展示的screenId
			delete curRegion.screenDepth;
			delete curRegion.cachedDom;//缓存screen dom
			delete curRegion.screenMap;
			delete curRegion.loadedScreenMap;//记录已经加载的screen ,screenName -> [para ->screenUUID]
			
			var loadedRegionIds = curRegion.screenIdMap.keySet();
			for(var i = 0 ; i<loadedRegionIds.length ;i++){
				RegionUtil.getRegionById(loadedRegionIds[i]).release();
			}
			
			delete curRegion.screenIdMap;//screen uuid  ->  screenInfo
			window.removeEventListener("popstate",REGION.onPopstate);
			delete curRegion.ignoreSearchPara;
		}
}


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION");
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.onRelease = REGION.onRelease;
	staticRegion.renderRegion();
})
</script>

