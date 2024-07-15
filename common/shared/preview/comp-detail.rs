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
    text-align: left;
}

#REGION>.content{
	
}

</style>

<div id="REGION" class="hidden" no-footer>
<!-- <input type="file" class="hidden fileChooser" onchange="REGION.onFileSelect(this)"> -->
	<div class="control">
		<span>目标组件：</span><span class="curRegionSrc"></span><br/><br/><br/>
		<button class="btn" data-type="default" normal onclick="REGION.previewLoalComponent(this)">替换为本地组件</button>
		<button class="btn" data-type="warning" normal onclick="REGION.previewRemoteComponent(this)">替换为远程组件</button>
		<button class="btn" data-type="default" style="float: right;" onclick="REGION.viewComponentRepo(this)"><i class="fa-light fa-magnifying-glass"></i>&nbsp;&nbsp;中央仓库组件</button>
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
			var targetRegion = curRegion.replaceTarget;
        	var targetRegionUrl = targetRegion.rcsFile;
        	var targetRegionHolder =$(targetRegion.getRegionDivElem().parentNode.parentNode);
        	var targetRegionId = targetRegion.regionId;
        	var paraMap = targetRegion.paraMap;
        	targetRegion.release();
         	var loadRegionPromise = RS.loadRegion(targetRegionHolder,RS.appendParaMapToUrl(regionUrl,paraMap),targetRegionId);
			
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
            	var targetRegion = curRegion.replaceTarget;
            	var targetRegionUrl = targetRegion.rcsFile;
            	var targetRegionHolder =$(targetRegion.getRegionDivElem().parentNode.parentNode);
            	var targetRegionId = targetRegion.regionId;
            	targetRegion.release();
             	var loadRegionPromise = RS.loadRegionByContent(targetRegionHolder,targetRegionUrl,this.result,null,targetRegionId);
//             	loadRegionPromise.done(function(loadedRegion){
//             		REGION.updateFileChooser(curRegion);
//             	});
             	RS.closeModalWindow();
            }
		},
		afterRenderData:function(){
			var curRegion = this;
			var targetRegionId = curRegion.paraMap.get("targetRegionId");
			var targetRegion = RS.getRegionById(targetRegionId);
			curRegion.find(".curRegionSrc").text(targetRegion.rcsFile);
			curRegion.replaceTarget = targetRegion;//记录要替换的组件
			console.log($(curRegion.replaceTarget.getRegionDivElem().parentNode.parentNode))
			REGION.updateFileChooser(curRegion);
		}
};


RS.ready(function(){
	var region = RS.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.loadRemoteRegion = REGION.loadRemoteRegion;
	region.renderRegion();
})
</script>