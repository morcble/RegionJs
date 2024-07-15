<style type="text/css">
#REGION>.table-body{
	height:100%;
	padding: 0rem 0.75rem 0.75rem 0.75rem;
}

#REGION>.table-body>.datas{
	height:100%;
	min-height: 0;
}
</style>
<!-- 
1. 最大行数限制  默认为20  
2. 类型分为 尾部追加和头部追加。添加数据时，追加到列表末尾或者头部
3. 尾部追加,当窗口已经显示最后一列的时候，往下滚动
4. 头部追加,当窗口已经显示第一列的时候，往上滚动
5. 数据增加有动画效果 
6. 默认会自动滚动 
-->

<div id="REGION" class="hidden">
		<div class="table-body" no-index>
            <div class="datas">
            	<div class="start"></div>
            	
            	<div class="end"></div>
            </div> 
       	</div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			
			if(!curRegion.inited){
				curRegion.inited = true;
				curRegion.headPointor = -1;//首指针
				curRegion.tailPointor = 0;//尾指针
				
				var autoScroll = curRegion.paraMap.get("autoScroll");
				if(autoScroll=="false")curRegion.autoScroll = false;
				else if(autoScroll==null||autoScroll=="true")curRegion.autoScroll = true;
				
				var maxRowNum = curRegion.paraMap.get("maxRowNum");
				if(maxRowNum==null)maxRowNum = 10;
				curRegion.maxRowNum = parseInt(maxRowNum);
				
				var append = curRegion.paraMap.get("append");
				if(append==null||append=="bottom")curRegion.append = "bottom";//底部追加
				else curRegion.append = "top";//头部追加
				
				var rowTemplateObj = $(curRegion.paraMap.get("rowTemplate"));
				rowTemplateObj.attr("row","$rowindex");
				var regionAttrArray = REGION.replaceRegionAttr(rowTemplateObj);
				curRegion.rowTemplate = rowTemplateObj.prop("outerHTML");
				curRegion.regionAttrArray = regionAttrArray;
			}
			
		},
		replaceRegionAttr:function(domNode){
			var attrArray = [];
			var regionAttrVal = domNode.attr("region-attr");
			if(regionAttrVal!=null){
				domNode.attr("region-attr","r_$num_"+regionAttrVal);
				attrArray.push(regionAttrVal);
			}
			domNode.children().each(function(index,child){
				attrArray = attrArray.concat(REGION.replaceRegionAttr($(child)));
			})
			return attrArray;
		},
		addRow:function(arrayData){
			var curRegion = this;
			if(arrayData==null)return;
			
			if(!(arrayData instanceof Array)){
				arrayData = [arrayData];
			}
			else{
				if(curRegion.append=="top")arrayData.reverse();
			}
			
			if(arrayData.length==0)return;
			
			if(curRegion.cacheRowCount==null){
				curRegion.cacheRowCount = 0;
			}
			else {
				curRegion.cacheRowCount+=arrayData.length;//记录和上次刷新相比累积偏移的行数
			}
			
			if(curRegion.maxRowNum!=-1 && arrayData.length>curRegion.maxRowNum){
				arrayData.splice(0,(arrayData.length-curRegion.maxRowNum));
			}
			
			var dynamicHtml = "";
			var rowTemplate = curRegion.rowTemplate;
			for(var i = 0 ; i<arrayData.length;i++){
				var rowData = arrayData[i];
				curRegion.headPointor++;
				for(var j = 0 ; j<curRegion.regionAttrArray.length ;j++){
					curRegion.regionData["r_"+curRegion.headPointor+"_"+curRegion.regionAttrArray[j]] = rowData[curRegion.regionAttrArray[j]];
				}
// 				for(var x in rowData){
// 					curRegion.regionData["r_"+curRegion.headPointor+"_"+x] = rowData[x];
// 				}
				var tmpHtml = rowTemplate.replaceAll("\\$num",curRegion.headPointor).replace("$rowindex","r_"+curRegion.headPointor);
				dynamicHtml += tmpHtml;
			}
			if(curRegion.append=="top"){
				var targetNode = curRegion.find(".start");
				RS.insertAfter(curRegion,targetNode,dynamicHtml);
				
				if(curRegion.maxRowNum!=-1){
					//清理溢出的数据项目
					var dataTotalAmount = curRegion.headPointor - curRegion.tailPointor + 1;
					var exceedAmount = dataTotalAmount - curRegion.maxRowNum;
					if(exceedAmount>0){
						var childs = curRegion.find(".datas").children();
						var total = childs.length;
						for(var i = (total-2) ;i>(curRegion.maxRowNum);i--){
							var toRemoveNode = $(childs[i]);
							var rowNumber = toRemoveNode.attr("row");
							
							for(var j = 0 ; j<curRegion.regionAttrArray.length ;j++){
								delete curRegion.regionData[rowNumber+"_"+curRegion.regionAttrArray[j]];//
							}
							
							toRemoveNode.remove();
						}
						curRegion.tailPointor+=exceedAmount;//移动尾部指针
					}
				}
			}
			else{
				var targetNode = curRegion.find(".end");
				RS.insertBefore(curRegion,targetNode,dynamicHtml);
				
				if(curRegion.maxRowNum!=-1){
					//清理溢出的数据项目
					var dataTotalAmount = curRegion.headPointor - curRegion.tailPointor + 1;
					var exceedAmount = dataTotalAmount - curRegion.maxRowNum;
					if(exceedAmount>0){
						var childs = curRegion.find(".datas").children();
						for(var i = 1 ;i<(exceedAmount+1);i++){
							var toRemoveNode = $(childs[i]);
							var rowNumber = toRemoveNode.attr("row");
							
							for(var j = 0 ; j<curRegion.regionAttrArray.length ;j++){
								delete curRegion.regionData[rowNumber+"_"+curRegion.regionAttrArray[j]];//
							}
							
							toRemoveNode.remove();
						}
						curRegion.tailPointor+=exceedAmount;//移动尾部指针
					}
				}
			}
			
			if(curRegion.autoScroll)
				curRegion.calculateScroll();
		},
		calculateScroll:function(){//计算滚动条
			var curRegion = this;
			var now = new Date().getTime();
			
			if(curRegion.lastScrollTime==null){
				//第一次scroll
				curRegion.lastScrollTime = new Date().getTime();
				setTimeout(function(){
					REGION.subCalScroll(curRegion);
				})
				return;
			}
			
			//忽略时间非常接近的task
			var lastExcuteGap = now-curRegion.lastScrollTime;
			if(lastExcuteGap<1000){
				if(curRegion.scrollTask!=null){
					//console.log("terminal task")
					window.clearTimeout(curRegion.scrollTask);//if long time no update , can not terninate task
				}
			}	
			else{
				if(curRegion.scrollTask!=null) {
					//console.log("ignore new task")
					return;
				}
			}
			
			//console.log("new task")
			curRegion.scrollTask = setTimeout(function(){
				curRegion.scrollTask = null;
				curRegion.lastScrollTime = new Date().getTime();
				//console.log("scroll")
				REGION.subCalScroll(curRegion);
			},500);
			
		},
		subCalScroll:function(targetRegion){
			var cacheRowCount = targetRegion.cacheRowCount;
			//if(cacheRowCount==0)return;
			targetRegion.cacheRowCount = 0;//复位
			//执行UI计算
			var datasPart = targetRegion.find(".datas");
			if(targetRegion.rowHeight==null){
				var childs = datasPart.children();
				if(childs.length>2){
					targetRegion.rowHeight = $(childs[1]).height();
				}
			}
			
			if(targetRegion.append=="top"){//从列表上面追加
				if(datasPart.scrollTop()< (targetRegion.rowHeight*2)){
					//自动滚动到最上面
					datasPart.scrollTop(0);
				}
			}
			else{//从列表下面追加
				if(targetRegion.notFirstTimeCal==true){
					if((datasPart[0].scrollHeight - datasPart.scrollTop() - datasPart.height())< (targetRegion.rowHeight*(cacheRowCount+4))){////列表接近底部4行之内才会自动滚动。
						//自动滚动到最下面
						datasPart.animate({ scrollTop: datasPart[0].scrollHeight }, "slow");
					}
					//datasPart.animate({ scrollTop: datasPart[0].scrollHeight }, "slow");
				}
				else{
					datasPart.scrollTop(datasPart[0].scrollHeight);
				}
			}	
			targetRegion.notFirstTimeCal = true;//标记已经计算过了
		},
		setAutoScroll:function(enable){
			var curRegion = this;
			curRegion.autoScroll = enable;
			if(enable==true){//重置位置
				var datasPart = curRegion.find(".datas");
				if(curRegion.append=="top"){//从列表上面追加
					datasPart.scrollTop(0);
				}
				else{
					datasPart.scrollTop(datasPart[0].scrollHeight);
				}
			}
		}
};

RegionUtil.ready(function(){
	var region = RegionUtil.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.addRow = REGION.addRow;
	region.calculateScroll = REGION.calculateScroll;
	region.setAutoScroll = REGION.setAutoScroll;
	region.renderRegion();
})
</script>