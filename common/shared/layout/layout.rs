<style type="text/css">

</style>
<div id="REGION" class="hidden">
	
</div>	


<script type="text/javascript">
var REGION = {
		firstRender:true,
		renderContent:function(){
			var curRegion = this;
			var layoutIndex = curRegion.paraMap.get("layoutIndex");
			if(layoutIndex==null){
				curRegion.layoutIndex=1;
			}
			else{
				curRegion.layoutIndex=parseInt(layoutIndex);
			}

			var layoutDataStr = curRegion.paraMap.get("layoutData");
			var layoutData = RegionUtil.decodePara(layoutDataStr);
			
			curRegion.layoutData = layoutData;

			$(curRegion.getRegionDivElem()).resize(function() {//window resize监听
				var timeGap = 100;//第一次渲染的timer gap为100毫秒
				if(!REGION.firstRender){
					timeGap = 300;
				}
				
				if(curRegion.resizeTimer!=null){
					clearTimeout(curRegion.resizeTimer);
				}
				curRegion.resizeTimer = setTimeout(function(){
					curRegion.refreshLayout();
					curRegion.resizeLayout = null;
					REGION.firstRender = false;
				},timeGap)
			});
			
			$(curRegion.getRegionDivElem()).css("position","absolute").css("top","-100rem");
			return REGION.renderLayout(curRegion);
		},
		renderLayout:function(layoutRegion){
			var waitForLoad = $.Deferred();//等待所有加载完毕
			setTimeout(function(){
				var layoutData = layoutRegion.layoutData;
				var regionDivElem = layoutRegion.getRegionDivElem();
				//创建brace div
				var appendHtml = REGION.genLayoutHtml(layoutRegion);
				RegionUtil.setInnerHtml(layoutRegion,regionDivElem,appendHtml);

				//装载子region
				var childs = layoutData.childs;
				var regionId = null;
				var promiseArray = [];
				for(var i = 0 ; i<childs.length;i++){
					if(childs[i].id!=null){
						regionId = childs[i].id;
					}
					else{
						regionId = layoutRegion.regionId+"_"+i;
					}
					
					var para = childs[i].para;
					var paraMap = new HashMap();
					
					if(childs[i].type==0){//form
						if(para!=null){
							var paraObject = para;
							if(typeof paraObject=="string"){
								paraObject = JSON.parse(para);
							}
						
							for(x in paraObject){
								paraMap.put(x,paraObject[x]);
							}
						}
						
						var loadRegionPromise = RegionUtil.loadRegion(layoutRegion.find(".split"+i),RegionUtil.appendParaMapToUrl(childs[i].regionUrl,paraMap),regionId);
						promiseArray.push(loadRegionPromise);
					}
					else{//layout
						paraMap.put("layoutIndex",layoutRegion.layoutIndex+1);
						paraMap.put("para",para);
						paraMap.put("layoutData",childs[i]);
						var loadRegionPromise = RegionUtil.loadRegion(layoutRegion.find(".split"+i),RegionUtil.appendParaMapToUrl("common/shared/layout/layout.rs",paraMap),regionId);
						promiseArray.push(loadRegionPromise);
						loadRegionPromise.done(function(loadedRegion){
							var parentLayoutRegionId = loadedRegion.paraMap.get("parentLayoutRegionId");
						});
					}
				}
				
				
				var promiseCommand = "$.when(";
				for(var i = 0 ; i<promiseArray.length;i++){
					promiseCommand+="promiseArray["+i+"]";
					if(i<(promiseArray.length-1))promiseCommand+=",";
				}
				promiseCommand+=").done(function(){waitForLoad.resolve();if(typeof(layoutRegion.onload)==='function')layoutRegion.onload(layoutRegion);});"
				eval(promiseCommand);
				
				waitForLoad.done(function(){
					setTimeout(function(){
						$(layoutRegion.getRegionDivElem()).css("position","relative").css("top","");
					},300)
				})
			})
			return waitForLoad.promise();
		},
		calBraceSize:function(layoutRegion){
			var layoutData = layoutRegion.layoutData;
			
			var regionDivElem = layoutRegion.getRegionDivElem();
			//计算宽度或高度
			//layoutData.type  2:V ,  1:H
			var isVertical = layoutData.type==2?true:false;
			var autoBracesCount = 0;
			var braceSizes = [];
			var totalWidthOrHeight = 0;
			if(isVertical){
				totalWidthOrHeight = $(regionDivElem).height();
			}
			else{
				totalWidthOrHeight = $(regionDivElem).width()-0;//-20 减去滚动条宽度？
			}
				
			let baseFontSize=document.documentElement.style.fontSize.split('px')[0];
			var balance = totalWidthOrHeight;
			for(var i = 0 ; i<layoutData.splits.length;i++){
				var split = layoutData.splits[i];
				var tmpType = typeof(split);
				if(tmpType=="number"){//数字
					if(split==-1){
						braceSizes.push(-1);
						autoBracesCount++;
						continue;
					}
					else if(split>1){
					
					}
					else{
						split = totalWidthOrHeight*split;
					}
				}
				else{
					if(split.endsWith("rem")){//兼容rem
						split = split.replace("rem","")*baseFontSize;
					}
					else if(split.endsWith("px")){//兼容px
						split = split.replace("px","");
					}
					else if(split.endsWith("%")){//兼容%
						split = parseInt(split.replace("%",""));
						split = totalWidthOrHeight*split;
					}
				}
				
				
				braceSizes.push(split);
				balance-=split;
			}
			
			var autoSize = -1;
			if(autoBracesCount>0)
				autoSize = balance/autoBracesCount;//如果有多个-1 则均分
			
			//自适应长宽重新赋值
			if(autoSize!=-1){
				for(var i = 0 ; i<braceSizes.length;i++){
					if(braceSizes[i]==-1){
						braceSizes[i] = autoSize;
					}
				}
			}
			return braceSizes;
		},
		genLayoutHtml:function(layoutRegion){//初始加载及窗口变化时计算布局大小
			var layoutData = layoutRegion.layoutData;
			//计算宽度或高度
			//layoutData.type  2:V ,  1:H
			var isVertical = layoutData.type==2?true:false;
			var braceSizes = REGION.calBraceSize(layoutRegion);
			
			return REGION.buildHtml(braceSizes,isVertical,0);
		},
		buildHtml:function(braceSizes,isVertical,index){
			if(braceSizes.length==0)return "";
			if((braceSizes.length-1)<index)return "";
			
			if(braceSizes.length==1)return '<div class="split-wrapper"><div class="split split'+index+'"></div></div>';
			
			if((braceSizes.length-1)==index){
				return '<div class="split-wrapper"><div class="split split'+index+'"></div></div>';
			}
			else{
				var childHtml = REGION.buildHtml(braceSizes,isVertical,index+1);
				var tmpHtml = '<div class="split-wrapper wrapper'+index+'"';
                var tmpSize = braceSizes[index];
				if(isVertical){
					tmpHtml+=' style="padding-top:'+tmpSize+'px">';
					tmpHtml+='<div class="split split'+index+'" style="height:'+tmpSize+'px"></div>';
					tmpHtml+='<div class="split-balance">'+childHtml+'</div>';
				}
				else{
					tmpHtml+=' style="padding-left:'+tmpSize +'px">';
					tmpHtml+='<div class="split split'+index+'" style="width:'+tmpSize  +'px"></div>';
					tmpHtml+='<div class="split-balance">'+childHtml+'</div>';
				}
				tmpHtml+='</div>';
				return tmpHtml;
			}
		},
		refreshLayout:function(){//layoutData更改时,重新计算布局大小
			var layoutRegion = this;
			var layoutData = layoutRegion.layoutData;
			//计算宽度或高度
			//layoutData.type  2:V ,  1:H
			var isVertical = layoutData.type==2?true:false;
			var braceSizes = REGION.calBraceSize(layoutRegion);
			
			REGION.subRelayout(layoutRegion,braceSizes,isVertical,0);
		},
		subRelayout:function(layoutRegion,braceSizes,isVertical,index){
			if(braceSizes.length==0)return "";
			if((braceSizes.length-1)<index)return "";
			
			if(braceSizes.length==1)return;
			
			if((braceSizes.length-1)==index){
				return;
			}
			else{
				var childs = layoutRegion.layoutData.childs;
				var regionId = null;
				var promiseArray = [];
				for(var i = 0 ; i<childs.length;i++){
					if(childs[i].id!=null){
						regionId = childs[i].id;
					}
					else{
						regionId = layoutRegion.regionId+"_"+i;
					}
					
					var tmpSize = braceSizes[i];
					var wrapperObj = layoutRegion.find(".wrapper"+i);
					var splitObj = layoutRegion.find(".split"+i);
					if(isVertical){
// 						wrapperObj.css("padding-top",tmpSize+"px");
// 						splitObj.css("height",tmpSize+"px");
						wrapperObj.animate({"padding-top":tmpSize+"px"});
						splitObj.animate({"height":tmpSize+"px"});
					}
					else{
// 						wrapperObj.css("padding-left",tmpSize+"px");
// 						splitObj.css("width",tmpSize+"px");
						wrapperObj.animate({"padding-left":tmpSize+"px"});
						splitObj.animate({"width":tmpSize+"px"});
					}
					
					if(childs[i].type!=0){
						var childRegion = RS.getRegionById(regionId);
						childRegion.refreshLayout();
					}
				}
			}
		}
		
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION");
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.refreshLayout = REGION.refreshLayout;
	//staticRegion.onload  layout中所有组件加载完毕后回调此方法
	staticRegion.renderRegion();
})
</script>

	