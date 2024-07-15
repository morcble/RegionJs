<style type="text/css">


</style>

<div id="REGION" class="hidden">

				<div class="form-body ">
					<div class="col-xs-12 col-md-12 form-item-public  is-required">
						<message key="videoSrc" class="form-label"></message>
						<select region-attr="videoSrc" class="form-control region-editable" region-ds="VIDEO_SRC" onchange="REGION.onChangeVideoSrc">
						          <option value="" msgKey="global_msg.please_select"></option>
						</select>
					</div>
					
					<div class="col-xs-12 col-md-12 form-item-public  external hidden is-required">
						<message key="externalLink" class="form-label"></message>
						<input region-attr="externalLink" type="text" class="form-control region-editable" maxlength="300">						
					</div>
					
				
					<div class="col-xs-12 col-md-12 form-item-public  internal hidden is-required">
						<message key="uploadVideo" class="form-label"></message>
						
						<input region-attr="videoId" file-type="video" class="file region-editable" vendor="cloud" hint="上传视频" limit-size="1" paras="maxsize=3000000;accept=*">				
					</div>
					<div class="col-xs-12 col-md-12 form-item-public ">
						<message key="width" class="form-label"></message>
						<input region-attr="width" type="text" class="form-control region-editable" oninput="value=value.replace(/[^\d]/g,'')" maxlength="4">				
					</div>
					<div class="col-xs-12 col-md-12 form-item-public ">
						<message key="height" class="form-label"></message>
						<input region-attr="height" type="text" class="form-control region-editable" oninput="value=value.replace(/[^\d]/g,'')" maxlength="4">						
					</div>
					<div class="col-xs-12 col-md-12 form-item-public  internal hidden">
						(限制大小3000MB)
					</div>
					
					<div class="col-xs-12 col-md-12 text-center" style="position: absolute;left: 0;bottom: 1rem;">
						<button class="btn"  data-type="primary" normal onclick="REGION.insertFileIntoCKEditor(event)"><message key="global_msg.save"></message></button>
						<button class="btn"  data-type="default" normal onclick="RegionUtil.closeModalWindow()"><message key="global_msg.cancel"></message></button>
					</div>
				</div>
		</div>
<script type="text/javascript">
var REGION = {
		onChangeVideoSrc:function(newVal){
			var curRegion = RegionUtil.getRegionByElem(this);
			if(newVal=="internal"){
				curRegion.find(".external").addClass("hidden");
				curRegion.find(".internal").removeClass("hidden");
			}
			else if(newVal=="external"){
				curRegion.find(".internal").addClass("hidden");
				curRegion.find(".external").removeClass("hidden");
			}
		},
		insertFileIntoCKEditor:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			var videoId = curRegion.getVideoId();
			var videoSrc = curRegion.getVideoSrc();
			var width = curRegion.getWidth();
			var height = curRegion.getHeight();
			if(videoSrc==null){
				RegionUtil.alert("请选择视频来源");
				return;
			}
			
			if(videoSrc=="external"){
				var externalLink = curRegion.getExternalLink();
				if(externalLink==null){
					RegionUtil.alert("请填写外链地址");
					return;
				}

				var ckEditorName = curRegion.paraMap.get("ckEditorName");
				var editor = CKEDITOR.instances[ckEditorName];
				
				var videoElement = editor.document.createElement( 'img' );
	        	videoElement.setAttribute("src",Config.frontendPath+"images/video.jpg");
	        	videoElement.setAttribute("class","extvideo");
	        	videoElement.setAttribute("id",externalLink);
	        	
	        	var styleStr = "";
	    		
	        	if(height!=null){
	    			styleStr="height:"+height+"px";
	    		}
	        	
	    		if(width!=null){
	    			if(height==null)styleStr+="width:"+width+"px";
	    			else styleStr+=";width:"+width+"px";
	    		}
	    		
	    		if(styleStr!=""){
	    			videoElement.setAttribute("style",styleStr);
	    		}
	        	
	        	editor.insertElement(videoElement);
			}
			else if(videoSrc=="internal"){
				var videoId = curRegion.getVideoId();
				if(videoId==null){
					RegionUtil.alert("请选择视频");
					return;
				}
				
				var ckEditorName = curRegion.paraMap.get("ckEditorName");
				var editor = CKEDITOR.instances[ckEditorName];
				
				var videoElement = editor.document.createElement( 'img' );
	        	videoElement.setAttribute("src",Config.frontendPath+"images/video.jpg");
	        	videoElement.setAttribute("class","video");
	        	videoElement.setAttribute("id",videoId);
	        	
	        	var styleStr = "";
	    		
	        	if(height!=null){
	    			styleStr="height:"+height+"px";
	    		}
	        	
	    		if(width!=null){
	    			if(height==null)styleStr+="width:"+width+"px";
	    			else styleStr+=";width:"+width+"px";
	    		}
	    		
	    		if(styleStr!=""){
	    			videoElement.setAttribute("style",styleStr);
	    		}
	        	
	        	editor.insertElement(videoElement);
			}
			
			
			
        	RegionUtil.closeModalWindow();
		}/* ,
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			
		} */
};


RegionUtil.ready(function(){
	var res = {
			message:{
				dft:{
					uploadVideo:"选择视频",
					width:"宽度",
					height:"高度",
					videoSrc:"视频来源",
					externalLink:"视频链接"
				}
			},
			dataList:{
				dft:{
					VIDEO_SRC:[{"text":"自有视频","value":"internal"},{"text":"外链视频","value":"external"}]
				}
			}
	};
	var formRegion = RegionUtil.newFormRegion("#REGION",res);
	//formRegion.afterRenderData = REGION.afterRenderData;
	formRegion.renderRegion();
})
</script>



