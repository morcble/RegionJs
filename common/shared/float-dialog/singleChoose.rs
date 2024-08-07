<!-- 单选 -->
<style type="text/css">
#REGION{
	padding: 0 0;
	max-height:18rem;
	overflow: auto;
}

#REGION>.item{
	line-height: 2.5rem;
    font-size: 1rem;
    border-bottom: 1px solid #f2f2f2;
    padding: 0 0.9rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    flex-wrap: wrap;
}
#REGION>.item>.checkitem{
    margin: 0 0.5rem 0.2rem 0;
}

</style>

<div id="REGION" class="hidden background-ff" data-type="border"></div>


<script type="text/javascript">
var REGION = {
		renderDialogData:function(listData,selectedItemArray){//渲染查询结果  必须实现的接口1
			var curRegion = this;
			var rootDiv = curRegion.getRegionDivElem();
			if(listData.length==0){
				$(rootDiv).html('<div class="item">暂无数据</div>');
				return;
			}
			var rowTemplate = '<div class="item" value="${value}" onclick="REGION.toggleItem(this)"><input class="checkitem" type="radio"><span class="text-part">${text}<span></div>';
			var dynamicHtml = RegionUtil.wrapList(rowTemplate,listData);
			
			
			RS.setInnerHtml(curRegion,rootDiv,dynamicHtml);
			var selectedVal = null;
			if(selectedItemArray!=null&&selectedItemArray.length>0){
				selectedVal = selectedItemArray[0].value;
			}
			
			$(rootDiv).children(".item").each(function(){
				var checkItem = $(this).children(".checkitem");
				if(selectedVal==this.getAttribute("value")){
					checkItem.prop("checked",true);
				}
				else{
					checkItem.prop("checked",false);
				}
				
				checkItem.click(function(evt){
					var checked = $(this).prop("checked");
					$(this).prop("checked",!checked);
					
					if(checked==true){
						$(this).parent().siblings().children(".checkitem").prop("checked",false);
					}
				})
			})
		},
		uncheckValue:function(targetVal){//点击弹窗里的选项，取消选中状态    必须实现的接口2
			var curRegion = this;
			var rootDiv = curRegion.getRegionDivElem();
			$(rootDiv).children(".item").each(function(){
				if(this.getAttribute("value")==targetVal){
					var checkItem = $(this).children(".checkitem");
					checkItem.prop("checked",false);
					return false;//终止循环
				}
			});
		},
		toggleItem:function(itemElem){//改变选中状态
			var curRegion = RS.getRegionByElem(itemElem);
			if(typeof curRegion.onClickItem === "function"){
				var checkItem = $(itemElem).children(".checkitem");
				var textPart = $(itemElem).children(".text-part");
				var checked = checkItem.prop("checked");
				var resultCheck = !checked;
				checkItem.prop("checked",resultCheck);
				
				if(resultCheck==true){
					checkItem.parent().siblings().children(".checkitem").prop("checked",false);
				}
				
				var allowMultiple = false;
				curRegion.onClickItem(resultCheck,itemElem.getAttribute("value"),textPart.text(),allowMultiple);
			}
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.renderDialogData = REGION.renderDialogData;
	staticRegion.uncheckValue = REGION.uncheckValue;
	staticRegion.renderRegion();
})
</script>