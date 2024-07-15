


<style type="text/css">
#REGION{
	overflow-x: hidden;
	padding-bottom: 0;
}
#REGION ul,li{
	margin: 0;
	padding: 0;
}
#REGION .draw-h3{
	width: 100%;
	text-align: center;
	padding: 1rem 0 0;
	float: left;
	margin: 0;
}
#REGION .no-border{
	border-top: 0;
}

#REGION .table-skin{
	width: 100%;
	float: left;
	display: flex;
	box-sizing: border-box;
	padding: 1rem;
}
#REGION .table-skin li{
	cursor: pointer;
	display: flex;
	justify-content:center;
	flex-wrap: wrap;
	padding:1rem;
	width: calc(100% / 4);
	min-height: 10rem;
	position: relative;

}
#REGION .table-skin li > label{
	padding:0;
	width: 100%;
	position: relative;
	cursor: pointer;
}
#REGION .background-f3 input[type="checkbox"] {
	display: none;
}
#REGION .background-f3 .checkmark {
	left: 0;
	right: 0;
	position: absolute;
	width: 100%;
	height: 100%;
}
#REGION .check-title{
	position: absolute;
	z-index: 11;
	text-align: center;
	padding: 1rem 0 ;
	left: 0;
	right: 0;
	font-size: 1.5rem;
}
#REGION .check-msg{
	position: absolute;
	z-index: 11;
	top: 50%;
	text-align: center;
	left: 0;
	right: 0;
	padding: 0 0.5rem;
    font-size: 0.9rem;
}

#REGION .theme-width {
  width: 100%;
  float: left;
  position: relative;
  height: 42rem;
  border-bottom: 2px solid #f2f2f2;
}
</style>

<div id="REGION" class="hidden" no-footer>
	<h3 class="draw-h3 no-border">主题风格</h3>
			<div  class="theme-width">

			</div>
	<h3 class="draw-h3">表格样式</h3>
	<ul class="table-skin">
		<li>
			<label  class="background-f3">
				<input type="checkbox" class="checkbox-input " data-check="checkborder"  onchange="REGION.TableClick(event,1)"/>
				<span class="checkmark"></span>
				<span class="check-title">带边框</span>
				<p class="check-msg">添加表格的边框线 border</p>
			</label>
		</li>
		<li>
			<label  class="background-f3">
				<input type="checkbox" class="checkbox-input " data-check="checkstripe"  onchange="REGION.TableClick(event,2)"/>
				<span class="checkmark"></span>
				<span class="check-title">斑马纹</span>
				<p class="check-msg">表格会间隔显示不同颜色，用于区分不同行数据 stripe</p>
			</label>
		</li>
		<li>
			<label  class="background-f3">
				<input type="checkbox" class="checkbox-input " data-check="checkcenter"  onchange="REGION.TableClick(event,3)"/>
				<span class="checkmark"></span>
				<span class="check-title">整体居中</span>
				<p class="check-msg">表格数据居中,默认居左</p>
			</label>
		</li>
		<li>
			<label  class="background-f3">
				<input type="checkbox" class="checkbox-input " data-check="checktableHeader"  onchange="REGION.TableClick(event,4)"/>
				<span class="checkmark"></span>
				<span class="check-title">表头居中</span>
				<p class="check-msg">表头居中,默认居左</p>
			</label>
		</li>

	</ul>

</div>


<script type="text/javascript">
var REGION = {
	checkedVale:{
		"checkborder":false,
		"checkstripe":false,
		"checkcenter":false,
		"checktableHeader":false
	},
	beforeRenderData:function (){
		let curRegion=this;

	},
	afterRenderData:function(){

        let curRegion=this;
		if (!curRegion.inited) {
			curRegion.inited = true

			var paraMap = new HashMap();
			paraMap.put("columns",1);
			paraMap.put("rows",1);
			var loadPromise = RS.loadRegion(curRegion.find(".theme-width"),RS.appendParaMapToUrl("common/shared/theme/theme-box.rs",paraMap));
			loadPromise.done(function(iconRegion){
				$.getJSON('common/shared/theme/theme.json',function(themData){
					iconRegion.renderSystemEntry(themData);
				})

			})
		}
		let theme =Number(localStorage.getItem("theme"))
		let tableBorder =JSON.parse(localStorage.getItem("tableBorder")) || ''
		if(theme){
			themeUtil.theme(theme);
		}if(tableBorder){
			REGION.checkedVale=tableBorder
			curRegion.find(".checkbox-input").each(function (element,dom){
				let item=$(dom);
				item.prop("checked",REGION.checkedVale[item.attr("data-check")])
			})

		}
	},

	//表格样式
	TableClick:function (event,number){
		var targetElem = RegionUtil.getEventTarget(event);
		let value=$(targetElem).prop("checked")
		switch (number){
			case 1:
				if(value===true){
					$(document.body).find(".table-body").attr("border","")
				}else{
					$(document.body).find(".table-body").removeAttr("border","")
				}
				REGION.checkedVale.checkborder=value
				break;
			case 2:
				if(value===true){
					$(document.body).find(".table-body").attr("stripe","")
				}else{
					$(document.body).find(".table-body").removeAttr("stripe","")
				}
				REGION.checkedVale.checkstripe=value
				break;
			case 3:
				if(value===true){
					$(document.body).find(".table-body").addClass("text-center")
				}else{
					$(document.body).find(".table-body").removeClass("text-center")
				}
				REGION.checkedVale.checkcenter=value
				break;
			case 4:
				if(value===true){
					$(document.body).find(".table-left").attr("center","")
				}else{
					$(document.body).find(".table-left").removeAttr("center","")
				}
				REGION.checkedVale.checktableHeader=value
				break;
		}
		localStorage.setItem("tableBorder",JSON.stringify(REGION.checkedVale))
	},
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.afterRenderData = REGION.afterRenderData;
	formRegion.beforeRenderData= REGION.beforeRenderData;
	formRegion.renderRegion();
})
</script>

