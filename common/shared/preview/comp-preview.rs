<style type="text/css">
#REGION{
	padding-top: 4.6rem;
	position: relative;
}

#REGION>.control{
	position: absolute;
    top: 1rem;
    left: 0rem;
    width: 100%;
    padding-left: 1rem;
}

#REGION>.content{
	
}

</style>

<div id="REGION" class="hidden" no-footer>
<!-- <input type="file" class="hidden fileChooser" onchange="REGION.onFileSelect(this)"> -->
	<div class="control">
		<button class="btn" data-type="default" normal onclick="REGION.previewLoalComponent(this)">本地组件</button>
		<button class="btn" data-type="warning" normal onclick="REGION.previewRemoteComponent(this)">远程组件</button>
		<button class="btn switchModeBtn" data-type="danger" normal onclick="REGION.switchMode(this)"></button>
		<span class="url"></span>
		<button class="btn" data-type="default" style="float: right;" onclick="REGION.viewComponentRepo(this)"><i class="fa-light fa-magnifying-glass"></i>&nbsp;&nbsp;中央仓库组件</button>
	</div>
	<div class="match-parent">
		<div class="content match-parent" style="border-top: 1px dotted blue">
		
		</div>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		viewComponentRepo:function(elem){
			var curRegion = RS.getRegionByElem(elem);
			var paraMap = new HashMap();
			paraMap.put("url","https://rcsdn.cn/sys/rsrepo/public/public.html");
			var handler = openDrawerWindow(RS.appendParaMapToUrl("common/shared/preview/html-preview.rs",paraMap),"中央组件仓库","l");
		},
		updateFileChooser:function(curRegion){
			curRegion.find(".fileChooser").remove();
			var newInput = document.createElement('input');
			newInput.type = 'file';
			var fileInput = $(newInput).addClass("hidden").addClass("fileChooser").attr("region-id",curRegion.regionId);
			fileInput.insertBefore(curRegion.find(".control"));
			curRegion.find(".fileChooser").change(function(){
				REGION.onFileSelect(this);
			})
		},
		previewRemoteComponent:function(elem){
			var curRegion = RS.getRegionByElem(elem);
			var paraMap = new HashMap();
			paraMap.put("openerId",curRegion.regionId);
			var handler = openDrawerWindow(RS.appendParaMapToUrl("common/shared/preview/remote-comp-url.rs",paraMap),"远程组件地址","s");
		},
		loadRemoteRegion:function(regionUrl){
			var curRegion = this;
			RS.clearCache();
			curRegion.find(".url").text(regionUrl);
			var loadRegionPromise = RS.loadRegion(curRegion.find(".content"),regionUrl);
			loadRegionPromise.done(function(loadedRegion){
				REGION.updateFileChooser(curRegion);
        	});
			
		},
		previewLoalComponent:function(elem){
			var curRegion = RS.getRegionByElem(elem);
			curRegion.find(".fileChooser").click();
		},
		onFileSelect:function(elem){
			var curRegion = RS.getRegionByElem(elem);
			
			var selectedFile = elem.files[0];
			var reader = new FileReader();//这是核心,读取操作就是由它完成.
            reader.readAsText(selectedFile);//读取文件的内容,也可以读取文件的URL
            reader.onload = function () {
            	curRegion.find(".url").text(selectedFile.name);
            	var loadRegionPromise = RS.loadRegionByContent(curRegion.find(".content"),null,this.result);
            	loadRegionPromise.done(function(loadedRegion){
            		REGION.updateFileChooser(curRegion);
            	});
            }
		},
		afterRenderData:function(){
			var curRegion = this;
			//var paraName = this.paraMap.get("paraName");
			REGION.updateFileChooser(curRegion);
			var switchBtn = curRegion.find(".switchModeBtn");
			if(window.regionDetailHolder==null){
				switchBtn.text("开启组件替换模式");
			}
			else{
				switchBtn.text("退出替换模式");
			}
		},
		switchMode:function(elem){
			var curRegion = RS.getRegionByElem(elem);
			if(window.regionDetailHolder==null){
				RS.edit();
				$(elem).text("退出替换模式");
				RS.alert("组件替换模式已开启")
			}
			else{
				RS.quitEdit();
				$(elem).text("开启组件替换模式");
				RS.alert("组件替换模式已关闭")
			}
		}
		
};


RS.ready(function(){
	var region = RS.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.loadRemoteRegion = REGION.loadRemoteRegion;
	region.renderRegion();
})
</script>