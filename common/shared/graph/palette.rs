<style type="text/css">
.palette-block{
	overflow: hidden;
	margin-bottom: 15px;
    padding: 0px 5px 0px 5px;
   
    position: relative;
}

.palette-block>.block-control{
	left: 0.625rem;
    position: absolute;
    top: 6px;
}

.palette-block>.palette-title{
	padding: 0.3125rem;
	position: relative;
	margin-left: 1.25rem;
}

.palette-block>.palette-item-wrap{
	width: 50%;
	float: left;
	padding: 5px 5px;
}

.palette-block .palette-item label{
	cursor: move;
}

.palette-block .palette-item{
    text-align: center;
    padding: 5px 5px;
    cursor: move;
    
    
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.palette-block .palette-item:hover{
	background: #409eff;
	color:white;
}

.palette-block>.palette-item>.palette-item-icon{

}

.palette-block>.palette-item>.palette-item-label{

}

#REGION{
	
}

</style>

<div id="REGION" class="hidden">
	<div class="palette-block">
		<div class="palette-title">
			<message key="layout"></message>
		</div>
		<div class="block-control"><i class="fa fa-minus block-btn"></i></div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="spliter" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="spliter"></message></div>
			</div>
		</div>
<!-- 		<div class="palette-item-wrap"> -->
<!-- 			<div class="palette-item" item-type="spliter" draggable="true"> -->
<!-- 				<div class="palette-item-icon"></div> -->
<!-- 				<div class="palette-item-label">标签页</div> -->
<!-- 			</div> -->
<!-- 		</div> -->
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="section" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="section"></message></div>
			</div>
		</div>
	</div>

	<div class="palette-block">
		<div class="palette-title">
			<message key="inputsTitle"></message>
		</div>
		<div class="block-control"><i class="fa fa-minus block-btn"></i></div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="text" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="input"></message></div>
			</div>
		</div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="m-text" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="inputArea"></message></div>
			</div>
		</div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="s-select" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="dropDown"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="m-select" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="muti-dropDown"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="s-choose" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="Radio"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="m-choose" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="CheckBox"></message></div>
			</div>
		</div>
	
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="transfer" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="transfer"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="edittable" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="edittable"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="date" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="date"></message></div>
			</div>
		</div>

		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="timepicker" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="timepicker"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="star" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="star"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="switch" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="switch"></message></div>
			</div>
		</div>
		
	
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="file" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="file"></message></div>
			</div>
		</div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="img" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="img"></message></div>
			</div>
		</div>
		
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="slider" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="slider"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="colorpicker" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="colorpicker"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="numcounter" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="numcounter"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="richinput" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="richinput"></message></div>
			</div>
		</div>
		
		
		
	</div>
	
	<div class="palette-block">
		<div class="palette-title">
			<message key="display-wiget"></message>
		</div>
		<div class="block-control"><i class="fa fa-minus block-btn"></i></div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="view-text" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="display-label"></message></div>
			</div>
		</div>
		
		<div class="block-control"><i class="fa fa-minus block-btn"></i></div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="view-ds" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="display-option"></message></div>
			</div>
		</div>
		
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="view-barcode" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="barcode"></message></div>
			</div>
		</div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="view-qrcode" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="qrcode"></message></div>
			</div>
		</div>
<!-- 		<div class="palette-item-wrap"> -->
<!-- 			<div class="palette-item" item-type="view-code" draggable="true"> -->
<!-- 				<div class="palette-item-icon"></div> -->
<!-- 				<div class="palette-item-label">代码块</div> -->
<!-- 			</div> -->
<!-- 		</div>		 -->
	</div>
	
	<div class="palette-block">
		<div class="palette-title">
			<message key="components"></message>
		</div>
		<div class="block-control"><i class="fa fa-minus block-btn"></i></div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="region" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="customized-comp"></message></div>
			</div>
		</div>
	</div>
	
	
	<div class="palette-block">
		<div class="palette-title">
			<message key="all-are-comps"></message>
		</div>
		<div class="block-control"><i class="fa fa-minus block-btn"></i></div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="region" sub-type="tree" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="tree"></message></div>
			</div>
		</div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="region" sub-type="selectable-tree" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="select-tree"></message></div>
			</div>
		</div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="region" sub-type="menu" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label"><message key="v-menu"></message></div>
			</div>
		</div>
		<div class="palette-item-wrap">
			<div class="palette-item background-f3" item-type="region" sub-type="editor" draggable="true">
				<div class="palette-item-icon"></div>
				<div class="palette-item-label" title="本设计器也是一个组件"><message key="editor"></message></div>
			</div>
		</div>
	</div>
	
	
</div>	


<script type="text/javascript">
var REGION = {
		renderContent:function(){
			//var paraName = this.paraMap.get("paraName");
			var curRegion = this;
			
			curRegion.find(".palette-item").each(function(){
			    this.addEventListener("dragstart",REGION.onDragStart);
				this.addEventListener("dragend",REGION.onDragEnd);
			})
			
			curRegion.find(".block-btn").click(function(){
				var blockBtn = $(this);
				if(blockBtn.hasClass("fa-minus")){
					blockBtn.removeClass("fa-minus").addClass("fa-plus");
					blockBtn.closest(".palette-block").find(".palette-item-wrap").addClass("hidden");
				}
				else{
					blockBtn.addClass("fa-minus").removeClass("fa-plus");
					blockBtn.closest(".palette-block").find(".palette-item-wrap").removeClass("hidden");
				}
			});
			
			curRegion.find(".palette-item").dblclick(function(evt){
				//双击添加控件到编辑器
				var itemType = this.getAttribute("item-type");
				var subType = this.getAttribute("sub-type")
				var title = $(this).find("LABEL").text()+" :";
				
				var editorRegion = RegionUtil.getOuterRegion(curRegion);
				
				var contentRegion = editorRegion.contentRegion;
				
				var newFormObject = contentRegion.createEditableDataBlockFromPalette(contentRegion,itemType,subType,title,false);
				var regionDivObj = $(contentRegion.getRegionDivElem());
				var exsitsBlocks = regionDivObj.children(".data-block-wrapper");
				if(exsitsBlocks.length==0){
					regionDivObj.append(newFormObject);
				}
				else{
					newFormObject.insertAfter($(exsitsBlocks[exsitsBlocks.length-1]));
				}
				
				//解析region标签  regionform的方法
				editorRegion.contentRegion.resolveInnerTags(editorRegion.dragObject[0]);
				//添加事件
				editorRegion.contentRegion.registerEventForBlock(editorRegion.dragObject);
				editorRegion.dragObject.click();//模拟点击事件
				editorRegion.dragObject = null;
			});
		},
		//拖动block
		onDragStart:function(evt){
			var targetElem = RegionUtil.getEventTarget(evt);
			
			var itemType = targetElem.getAttribute("item-type");
			var subType = targetElem.getAttribute("sub-type");
			var title = $(targetElem).find("LABEL").text()+" :";
			
			var curRegion = RegionUtil.getRegionByElem(targetElem);
			var editorRegion = RegionUtil.getOuterRegion(curRegion);
			editorRegion.dragFrom = "palette";//设置拖动源
			editorRegion.dragItemType = itemType;//设置拖动源
			editorRegion.dragItemSubType = subType;//设置拖动源
			editorRegion.dragItemTitle = title;//设置拖动源
			
			//状态复位
			editorRegion.insertedDom = false;
			
			evt.dataTransfer.setData("Text","");//兼容firefox
		},
		onDragEnd:function(evt){
			evt.preventDefault();
			console.log("onDragEnd:")
			var curRegion = RegionUtil.getRegionByEvent(evt);
			//清理
			var editorRegion = RegionUtil.getOuterRegion(curRegion);
			if(editorRegion.dragObject!=null){
				editorRegion.dragObject.find(".block-control").removeClass("hidden");
				editorRegion.dragObject.removeClass("minimized");
				clearInterval(curRegion.dragInterval);
				
				//解析region标签  regionform的方法
				editorRegion.contentRegion.resolveInnerTags(editorRegion.dragObject[0]);
				//添加事件
				editorRegion.contentRegion.registerEventForBlock(editorRegion.dragObject);
				editorRegion.dragObject.attr("item-type",editorRegion.dragItemType);
				editorRegion.dragObject.click();//模拟点击事件
				editorRegion.dragObject = null;
			}
		}
};


RegionUtil.ready(function(){
	var res = {
			message:{
				dft:{
					"layout":"布局",
					"spliter":"水平分割线",
					"section":"段落",
					"inputsTitle":"可输入控件 (双击或拖拽)",
					"input":"输入框",
					
					"inputArea":"多行输入框",
					"dropDown":"下拉单选",
					"muti-dropDown":"下拉多选",
					"Radio":"单选",
					"CheckBox":"多选",
					
					
					"transfer":"穿梭框",
					"edittable":"可编辑表格",
					"date":"日期",
					"timepicker":"时间",
					"star":"评分",
					"switch":"开关",
					
					"file":"文件",
					"img":"图片",
					"slider":"滑块",
					"colorpicker":"拾色器",
					"numcounter":"计数器",
					
					"richinput":"富文本",
					"display-wiget":"显示控件",  
					"display-label":"文字",
					"display-option":"选项结果",
					"barcode":"条形码",
					"qrcode":"二维码",
					
					"components":"嵌套组件",  
					"customized-comp":"自定义组件",
					"all-are-comps":"一切皆组件",
					"tree":"普通树",
					"select-tree":"可选树",
					"v-menu":"竖向菜单",
					"editor":"本设计器套娃",
				},
				en:{
					"layout":"Layout",
					"spliter":"Spliter",
					"section":"Section",
					"inputsTitle":"Inputs (Double click or drag)",
					"input":"Text",
					
					"inputArea":"TextArea",
					"dropDown":"Select",
					"muti-dropDown":"Muti-Select",
					"Radio":"Radio",
					"CheckBox":"CheckBox",
					
					
					"transfer":"Transfer",
					"edittable":"Edit Table",
					"date":"Date",
					"timepicker":"Time",
					"star":"Star",
					"switch":"Switch",
					
					"file":"File",
					"img":"Image",
					"slider":"Slider",
					"colorpicker":"Color",
					"numcounter":"Counter",
					
					"richinput":"Editor",
					"display-wiget":"Views",  
					"display-label":"Label",
					"display-option":"View Option",
					"barcode":"Barcode",
					"qrcode":"Qrcode",
					
					"components":"Components",  
					"customized-comp":"Customized",
					"all-are-comps":"Every thing is component",
					"tree":"Tree",
					"select-tree":"Select Tree",
					"v-menu":"Vertical Menu",
					"editor":"Comp Editor",
				}
			}
	};

	
	var staticRegion = RegionUtil.newStaticRegion("#REGION",res);
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.renderRegion();
})
</script>

