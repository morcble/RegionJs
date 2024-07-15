<style type="text/css">
foreignObject{
	user-select: none;
	padding: 8px 8px;
}

foreignObject:hover{
	cursor: cell;
	background: #f2f2f2;
}

foreignObject>div{
	background: #ffffff;
}

foreignObject>div:hover{
	cursor: grabbing;
}

/* #REGION .shadows{ */
/* 	pointer-events: none; */
/* } */

</style>

<div id="REGION" class="hidden">
<!-- 	移动 -->
<!-- 	多个移动 -->
<!-- 	连线 -->
<!-- 	连线后移动图形跟随 -->

<!-- 	缩放 -->
<!-- 	删除 -->
<!-- 	保存和加载 -->
<!-- 	复制 -->
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			var regionDom = this.getRegionDivElem();
			var curRegion = this;
			if(!curRegion.inited){
				curRegion.inited = true;
				var drawBoard = $.rsplugin.graph.initDrawBoard(curRegion);
				var shapePara = {x:100,y:100,height:60,width:120}
				var loadShapePromise = drawBoard.addShape("common/shared/graph/shapes/dom/rect.rs",shapePara);
				
				
				var shapePara = {x:300,y:300,height:120,width:120,type:"dataset",label:rowData.name,data:rowData}
				drawBoard.addShape("common/shared/graph/shapes/dom/rect.rs",shapePara);
				
				var shapePara = {x:400,y:500,height:120,width:120,type:"dataset",label:rowData.name,data:rowData}
				drawBoard.addShape("common/shared/graph/shapes/dom/rect.rs",shapePara);
			}
		}
		
};


RS.ready(function(){
	var curRegion = RS.newRegion("#REGION",null);
	curRegion.afterRenderData = REGION.afterRenderData;
	curRegion.renderRegion();
})
</script>