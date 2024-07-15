
/**
 * 4 component : FileUploader
 *
 */

/**
 * 文件上传工具类
 * @param wrapperObj
 * @param inputObj
 * @param fileObj
 * @param onUploaded每次上传完成回调
 * @returns
 */
function FileUploader(wrapperObj,inputObj,fileObj,onUploaded){
	var curFileUploader = this;
	this.wrapperObj = wrapperObj;
	this.inputObj = inputObj;//input控件
	this.fileObj = fileObj;//file 控件
	this.onUploaded = onUploaded;//file 控件
	this.id = RegionUtil.getUUID();

	this.parasMap = new HashMap();
	var paras = inputObj.attr("paras");
	var svcUrl = inputObj.attr("svc-url");
	var limitWidth = inputObj.attr("limit-width");//限制宽度
	if(limitWidth!=null)
		this.limitWidth = parseInt(limitWidth);
	
	console.log("limitWidth:"+this.limitWidth);
	
	if(svcUrl==null){
		svcUrl = Config.backendPath+"/RCS_Objectsystem";
	}
	this.svcUrl = svcUrl;

	if(paras!=null){
		var parasArray = paras.split(";");//限制参数
		for(var i = 0 ;i <parasArray.length ; i++){
			var tmpArray = parasArray[i].split("=");
			this.parasMap.put(tmpArray[0].trim(),tmpArray[1].trim());
		}
	}


	this.wrapperObj.find(".attach-btn").click(function(){
		curFileUploader.fileObj.click();
	});

	this.fileObj.change(function(event){
		var validateResult = curFileUploader.validation(event);
		if(validateResult){//通过验证才进行上传
			if (curFileUploader.wrapperObj.hasClass("img-wrapper")) {
				var plusView = curFileUploader.wrapperObj.find(".plus-view");
				var imgView = plusView.children(".img-view");
				imgView.removeClass("hidden");
				var previewImg = imgView.find(".preview");
				try{
					var reader=new FileReader();
			        reader.readAsDataURL(curFileUploader.fileObj[0].files[0]);  
			        reader.onload=function(){
			        	previewImg[0].src=reader.result;
			        	if(curFileUploader.limitWidth!=null){//需要缩放和裁剪
			        		setTimeout(function(){
			        			var newImageData = curFileUploader.compress(previewImg[0], curFileUploader.limitWidth);
			        			previewImg[0].src = newImageData;
			        			curFileUploader.compressedImageData = newImageData;
			        			
			        			plusView.children(".progress-cover").removeClass("hidden");
						        plusView.children(".hint").addClass("hidden");
						        plusView.children(".plus-btn").addClass("hidden");
						        var status = curFileUploader.inputObj.attr("status");
								if(status=="uploading"){//已经在上传中
									RegionUtil.alert(global_msg.file_is_uploading);
								}
								else{
									curFileUploader.inputObj.attr("status","uploading")//标记为正在上传
									curFileUploader.uploadFile();
								} 
				        	});
			        	}
			        }
			        
			        if(curFileUploader.limitWidth!=null)return;

			        plusView.children(".progress-cover").removeClass("hidden");
			        plusView.children(".hint").addClass("hidden");
			        plusView.children(".plus-btn").addClass("hidden");
				}
				catch(e){
					console.log(e)
				}
			}
			else if (curFileUploader.wrapperObj.hasClass("video-wrapper")) {
				var plusView = curFileUploader.wrapperObj.find(".plus-view");
				var videoView = plusView.children(".video-view");
				videoView.removeClass("hidden");
				var previewImg = videoView.find(".preview");
				try{
					var reader=new FileReader();
			        reader.readAsDataURL(curFileUploader.fileObj[0].files[0]);
			        reader.onload=function(){
			        	previewImg[0].src=reader.result;
			        	if(curFileUploader.limitWidth!=null){//需要缩放和裁剪
			        		setTimeout(function(){
			        			previewImg[0].src = newImageData;
			        			curFileUploader.compressedImageData = newImageData;
			        			
			        			plusView.children(".progress-cover").removeClass("hidden");
						        plusView.children(".hint").addClass("hidden");
						        plusView.children(".plus-btn").addClass("hidden");
						        var status = curFileUploader.inputObj.attr("status");
								if(status=="uploading"){//已经在上传中
									RegionUtil.alert(global_msg.file_is_uploading);
								}
								else{
									curFileUploader.inputObj.attr("status","uploading")//标记为正在上传
									curFileUploader.uploadFile();
								} 
				        	});
			        	}
			        }
			        
			        if(curFileUploader.limitWidth!=null)return;

			        plusView.children(".progress-cover").removeClass("hidden");
			        plusView.children(".hint").addClass("hidden");
			        plusView.children(".plus-btn").addClass("hidden");
				}
				catch(e){
					console.log(e)
				}
			}
			else if (curFileUploader.wrapperObj.hasClass("file-wrapper")) {
				var plusView = curFileUploader.wrapperObj.find(".plus-view");
				curFileUploader.wrapperObj.find(".ctrols>.attach-btn").addClass("hidden");

				var progressCover = plusView.children(".progress-cover");

				var file = curFileUploader.fileObj[0].files[0];
				progressCover.children(".file-info").text(curFileUploader.fileObj[0].files[0].name);
				progressCover.children(".progress-block").text("");
				progressCover.removeClass("hidden");
			}
			

			var status = curFileUploader.inputObj.attr("status");
			if(status=="uploading"){//已经在上传中
				RegionUtil.alert(global_msg.file_is_uploading);
			}
			else{
				curFileUploader.inputObj.attr("status","uploading")//标记为正在上传
				curFileUploader.uploadFile();
			}
		}
	});

	curFileUploader.initWithExsitData();

}

/**
 * 给已有的图片或者文件添加删除事件
 */
FileUploader.prototype.initWithExsitData = function(){
	var curFileUploader = this;
	var wrapperObj = curFileUploader.wrapperObj;
	if (wrapperObj.hasClass("img-wrapper")) {
		var uploadedCount = wrapperObj.find(".uploaded-imgs").children().length-1;
		var limitSize = wrapperObj.find(".wrapped").attr("limit-size");
		if(limitSize==null)limitSize=1;
		else limitSize = parseInt(limitSize);

		var plusView = wrapperObj.find(".plus-view");
		if(uploadedCount>=limitSize){
			plusView.addClass("hidden");
		}
		else{
			plusView.removeClass("hidden");
		}

		wrapperObj.find(".img-view>.preview").click(function(event){
			var paraMap = new HashMap();
			paraMap.put("imgBase64",$(this).attr("src"));
			openDrawerWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/img-preview.rs",paraMap), "图片预览","s");
			//RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/img-preview.rs",paraMap), "图片预览",2,0.7,0.7,null,null,false,true);
		});

		wrapperObj.find(".uploaded-imgs .delete-btn").click(function(event){
			curFileUploader.deleteImage(this);
		});
	}
	else if (wrapperObj.hasClass("video-wrapper")) {
		var uploadedCount = wrapperObj.find(".uploaded-videos").children().length-1;
		var limitSize = wrapperObj.find(".wrapped").attr("limit-size");
		if(limitSize==null)limitSize=1;
		else limitSize = parseInt(limitSize);

		var plusView = wrapperObj.find(".plus-view");
		if(uploadedCount>=limitSize){
			plusView.addClass("hidden");
		}
		else{
			plusView.removeClass("hidden");
		}

		wrapperObj.find(".video-view>.preview").click(function(event){
			var paraMap = new HashMap();
			paraMap.put("url",$(this).attr("src"));
			openDrawerWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/img-preview.rs",paraMap), "图片预览","s");
			//RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/video-preview.rs",paraMap), "视频预览",2,0.7,0.7,null,null,false,true);
		});

		wrapperObj.find(".uploaded-videos .delete-btn").click(function(event){
			curFileUploader.deleteVideo(this);
		});
	}
	else if (wrapperObj.hasClass("file-wrapper")) {
		var uploadedCount = wrapperObj.find(".uploaded-files").children().length-1;
		var limitSize = wrapperObj.find(".wrapped").attr("limit-size");
		if(limitSize==null)limitSize=1;
		else limitSize = parseInt(limitSize);
		if(uploadedCount>=limitSize){
			wrapperObj.find(".ctrols>.attach-btn").addClass("hidden");
		}
		else{
			wrapperObj.find(".ctrols>.attach-btn").removeClass("hidden");
		}

		wrapperObj.find(".uploaded-files .delete-btn").click(function(event){
			curFileUploader.deleteFile(this);
		});

		wrapperObj.find(".uploaded-files .file-info").click(function(event){
			var paraMap = new HashMap();
			paraMap.put("url",this.getAttribute("file-url"));
			if(AndroidClient.isFromWebview){//app
				RegionUtil.gotoScreen("videoPreview",paraMap);
			}
			else{
				window.open(this.getAttribute("file-url"),"_blank");
				//var opennedWindow = RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/video-preview.rs",paraMap), "视频预览",2,0.7,0.7,null,null,false,true);
			}
		});

	}
}

/**
 * 文件压缩尺寸 image
 */
FileUploader.prototype.compress = function(sourceImgObj, maxWidth ,quality,outputFormat){  
	if(quality==null)quality=100;
	if(outputFormat==null)outputFormat="jpeg";//png
	
    var mimeType = "image/"+outputFormat;  
    var cvs = document.createElement('canvas');  
    //图片的真实高
    var finalHeight = sourceImgObj.naturalHeight;
    //图片的真实宽
    var finalWidth = sourceImgObj.naturalWidth;
    //当图片的宽度大于maxWidth，则将宽度压缩到maxWidth，高度按比例缩小
    if(parseInt(sourceImgObj.naturalWidth) > maxWidth){
    	var proportion = parseInt(sourceImgObj.naturalWidth)/maxWidth;
    	finalHeight = sourceImgObj.naturalHeight/proportion;
    	finalWidth = maxWidth;
    }
    //naturalWidth真实图片的宽度  
    cvs.width = finalWidth;  
    cvs.height = finalHeight;  
    var ctx = cvs.getContext("2d").drawImage(sourceImgObj, 0, 0,sourceImgObj.naturalWidth,sourceImgObj.naturalHeight,0,0,finalWidth,finalHeight);  
    return cvs.toDataURL(mimeType, quality/100);  
}

FileUploader.prototype.dataURItoBlob = function(dataurl){
    var arr = dataurl.split(','), mime = arr[0].match(/:(.*?);/)[1],
    bstr = atob(arr[1]), n = bstr.length, u8arr = new Uint8Array(n);
    while(n--){
      u8arr[n] = bstr.charCodeAt(n);
    }
    return new Blob([u8arr], {type:mime});
}

FileUploader.prototype.uploadFile = function(){
	var fileUploader = this;
	
	
	if(fileUploader.compressedImageData!=null){
		var blobData = fileUploader.dataURItoBlob(fileUploader.compressedImageData);
		fileUploader.compressedImageData = null;
		fileUploader.targetFileBlob = blobData;
		fileUploader.targetFileBlob.name = this.fileObj[0].files[0].name;
	}
	else{
		fileUploader.targetFileBlob = this.fileObj[0].files[0];
	}
	
	
	var vendor = this.inputObj.attr("vendor");
	if(vendor==null)vendor = Config.fileVendor;
	if(vendor==null)vendor="cloud";//默认为云上传
	
	var storePath = this.inputObj.attr("store-path");
	if(storePath!=null && !storePath.endsWith("/"))storePath=storePath+"/";

	var fileType = this.inputObj.attr("file-type");
	if(fileType==null)fileType="file";//默认为文件服务

	var jqueryDef = RegionUtil.calculateMD5(fileUploader.targetFileBlob);
	jqueryDef.done(function(md5){
		var reqObj = new Object();
		reqObj.md5 = md5;
		reqObj.vendor = vendor;
		reqObj.upToken = RegionUtil.getCookie("fileUpToken");//文件上传凭证
		reqObj.fileType = fileType;//文件上传凭证
		reqObj.storePath = storePath;//文件上传凭证

		RegionUtil.ajaxJsonTask(fileUploader.svcUrl+"/objectsystem/pre-upload","POST",reqObj,function(serverData,dataPara){
			var responseData = serverData.data;
			console.log(responseData);
			if(!responseData.recordExsit){
				//if(reqObj.upToken == null&&responseData.upToken!=null){
				if(responseData.upToken!=null){
					RegionUtil.setCookieWithExpire("fileUpToken",responseData.upToken,"s"+60)
					reqObj.upToken = responseData.upToken;
				}

				if(vendor=="local"){
					fileUploader.uploadFileToLocal(responseData.fileKeyPrefix,reqObj.md5,reqObj.fileType);
				}
				else if(vendor=="cloud"){
					fileUploader.uploadFileToCloud(responseData.fileKeyPrefix,reqObj.upToken,reqObj.md5,reqObj.fileType);
				}
			}
			else{//找到重复的文件
				//donothing
				//CoverPlugin.loadingComplete();
				//alert("todo");//文件重复上传校验
				fileUploader.inputObj.val(responseData.fileId);
				fileUploader.inputObj.attr("status",null)//删除正在上传标记
			}
		})
	});
}

FileUploader.prototype.deleteImage = function(targetElem){
	var fileUploader = this;
	var tmpImgView = $(targetElem).parent();
	var tmpFileId = tmpImgView.attr("file-id");
	tmpImgView.remove();

	var tmpFileIds = fileUploader.inputObj.val();
	var index = tmpFileIds.indexOf(tmpFileId);
	var newTmpFileIds;

	if(tmpFileIds==tmpFileId){
		newTmpFileIds="";
	}
	else{
		if(index==0){
			newTmpFileIds = tmpFileIds.substring(tmpFileId.length+1,tmpFileIds.length);
		}
		else{
			newTmpFileIds = tmpFileIds.substring(0,index-1)+tmpFileIds.substring(index+tmpFileId.length,tmpFileIds.length);
		}
	}

	fileUploader.inputObj.val(newTmpFileIds);

	var limitSize = fileUploader.inputObj.attr("limit-size");
	if(limitSize==null)limitSize=1;
	else limitSize = parseInt(limitSize);

	var uploadedCount = fileUploader.wrapperObj.find(".uploaded-imgs").children().length-1;

	var plusView = fileUploader.wrapperObj.find(".plus-view");
	if(uploadedCount<limitSize){
		plusView.removeClass("hidden");
	}
}

FileUploader.prototype.deleteVideo = function(targetElem){
	var fileUploader = this;
	var tmpImgView = $(targetElem).parent();
	var tmpFileId = tmpImgView.attr("file-id");
	tmpImgView.remove();

	var tmpFileIds = fileUploader.inputObj.val();
	var index = tmpFileIds.indexOf(tmpFileId);
	var newTmpFileIds;

	if(tmpFileIds==tmpFileId){
		newTmpFileIds="";
	}
	else{
		if(index==0){
			newTmpFileIds = tmpFileIds.substring(tmpFileId.length+1,tmpFileIds.length);
		}
		else{
			newTmpFileIds = tmpFileIds.substring(0,index-1)+tmpFileIds.substring(index+tmpFileId.length,tmpFileIds.length);
		}
	}

	fileUploader.inputObj.val(newTmpFileIds);

	var limitSize = fileUploader.inputObj.attr("limit-size");
	if(limitSize==null)limitSize=1;
	else limitSize = parseInt(limitSize);

	var uploadedCount = fileUploader.wrapperObj.find(".uploaded-videos").children().length-1;

	var plusView = fileUploader.wrapperObj.find(".plus-view");
	if(uploadedCount<limitSize){
		plusView.removeClass("hidden");
	}
}

FileUploader.prototype.deleteFile = function(targetElem){
	var fileUploader = this;
	var fileView = $(targetElem).parent();
	var tmpFileId = fileView.attr("file-id");
	fileView.remove();

	var tmpFileIds = fileUploader.inputObj.val();
	var index = tmpFileIds.indexOf(tmpFileId);
	var newTmpFileIds;

	if(tmpFileIds==tmpFileId){
		newTmpFileIds="";
	}
	else{
		if(index==0){
			newTmpFileIds = tmpFileIds.substring(tmpFileId.length+1,tmpFileIds.length);
		}
		else{
			newTmpFileIds = tmpFileIds.substring(0,index-1)+tmpFileIds.substring(index+tmpFileId.length,tmpFileIds.length);
		}
	}

	fileUploader.inputObj.val(newTmpFileIds);

	var limitSize = fileUploader.inputObj.attr("limit-size");
	if(limitSize==null)limitSize=1;
	else limitSize = parseInt(limitSize);

	var uploadedCount = fileUploader.wrapperObj.find(".uploaded-files").children().length-1;
	if(uploadedCount<limitSize){
		fileUploader.wrapperObj.find(".ctrols>.attach-btn").removeClass("hidden");
	}
}

FileUploader.prototype.uploadComplete = function(result){
	var fileUploader = this;
	fileUploader.inputObj.attr("status",null)//删除正在上传标记
	fileUploader.wrapperObj.find(".cancel-btn").addClass("hidden");

	var plusView = fileUploader.wrapperObj.find(".plus-view");

	if (fileUploader.wrapperObj.hasClass("img-wrapper")) {
		plusView.children(".hint").removeClass("hidden");
	    plusView.children(".plus-btn").removeClass("hidden");
		plusView.children(".progress-cover").addClass("hidden");

	} 
	if (fileUploader.wrapperObj.hasClass("video-wrapper")) {
		plusView.children(".hint").removeClass("hidden");
	    plusView.children(".plus-btn").removeClass("hidden");
		plusView.children(".progress-cover").addClass("hidden");

	}
	else if (fileUploader.wrapperObj.hasClass("file-wrapper")) {
		fileUploader.wrapperObj.find(".progress-cover").addClass("hidden");
	}


	if(result==2){//exception
		console.log("exception")
		if (fileUploader.wrapperObj.hasClass("img-wrapper")) {
			plusView.children(".img-view").addClass("hidden");
		}
		else if (fileUploader.wrapperObj.hasClass("video-wrapper")) {
			plusView.children(".video-view").addClass("hidden");
		}
	}
	else if(result==3){//cancel
		console.log("cancel");
		if (fileUploader.wrapperObj.hasClass("img-wrapper")) {
			plusView.children(".img-view").addClass("hidden");
		}
		else if (fileUploader.wrapperObj.hasClass("video-wrapper")) {
			plusView.children(".video-view").addClass("hidden");
		}
	}
	else{
		var fileIds = fileUploader.inputObj.val();
		if(fileIds==null)fileIds="";

		if(fileIds.length==0){
			fileIds = result.fileId;
		}
		else{
			fileIds +=","+ result.fileId;
		}

		fileUploader.inputObj.val(fileIds);
		//成功上传
		btn = fileUploader.wrapperObj.find(".delete-btn");
		btn.removeClass("hidden");

		//如果是图片
		if(fileUploader.wrapperObj.hasClass("img-wrapper")){
			var plusView = fileUploader.wrapperObj.find(".plus-view");
			var wrappedObj = fileUploader.wrapperObj.find(".wrapped");
			var imgView = plusView.children(".img-view").remove();
			imgView.attr("file-id",result.fileId);
			//插入img
			imgView.insertBefore(fileUploader.wrapperObj.find(".uploaded-imgs>.brace"));
			RegionUtil.appendHtml(RegionUtil.getRegionByElem(imgView[0]),imgView[0],'<div class="delete-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div>');

			//imgView = fileUploader.wrapperObj.find(".uploaded-imgs>.img-view:last");

			imgView.children(".preview").click(function(event){
				var paraMap = new HashMap();
				paraMap.put("imgBase64",$(this).attr("src"));
				RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/img-preview.rs",paraMap), "图片预览",2,0.7,0.7,null,null,false,true);
			});

			imgView.children(".delete-btn").click(function(event){
				fileUploader.deleteImage(this);
			});

			var limitSize = wrappedObj.attr("limit-size");
			if(limitSize==null)limitSize=1;
			else limitSize = parseInt(limitSize);

			RegionUtil.appendHtml(RegionUtil.getRegionByElem(plusView[0]),plusView[0],'<div class="img-view"><img class="preview"></div>');

			var uploadedCount = fileUploader.wrapperObj.find(".uploaded-imgs").children().length-1;
			if(uploadedCount>=limitSize){
				//已经达到允许上传的最大个数
				plusView.addClass("hidden");
			}
		}
		else if(fileUploader.wrapperObj.hasClass("video-wrapper")){
			var plusView = fileUploader.wrapperObj.find(".plus-view");
			var wrappedObj = fileUploader.wrapperObj.find(".wrapped");
			var videoView = plusView.children(".video-view").remove();
			videoView.attr("file-id",result.fileId);
			//插入video
			videoView.insertBefore(fileUploader.wrapperObj.find(".uploaded-videos>.brace"));
			RegionUtil.appendHtml(RegionUtil.getRegionByElem(videoView[0]),videoView[0],'<div class="delete-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div>');

			//videoView = fileUploader.wrapperObj.find(".uploaded-videos>.video-view:last");

			videoView.children(".preview").click(function(event){
				var paraMap = new HashMap();
				paraMap.put("url",$(this).attr("src"));
				RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/video-preview.rs",paraMap), "视频预览",2,0.7,0.7,null,null,false,true);
			});

			videoView.children(".delete-btn").click(function(event){
				fileUploader.deleteVideo(this);
			});

			var limitSize = wrappedObj.attr("limit-size");
			if(limitSize==null)limitSize=1;
			else limitSize = parseInt(limitSize);

			RegionUtil.appendHtml(RegionUtil.getRegionByElem(plusView[0]),plusView[0],'<div class="video-view"><video class="preview"></div>');

			var uploadedCount = fileUploader.wrapperObj.find(".uploaded-videos").children().length-1;
			if(uploadedCount>=limitSize){
				//已经达到允许上传的最大个数
				plusView.addClass("hidden");
			}
		}
		else if(fileUploader.wrapperObj.hasClass("file-wrapper")){
			var plusView = fileUploader.wrapperObj.find(".plus-view");
			var wrappedObj = fileUploader.wrapperObj.find(".wrapped");

			var fileItemHtml = '<div class="file-view" file-id="'+result.fileId+'"><div class="file-info" file-url="'+RegionUtil.fixUrl(result.url)+'">'+plusView.find(".file-info").text()+'</div><div class="delete-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div>';

			var braceObj = fileUploader.wrapperObj.find(".uploaded-files>.brace");
			RegionUtil.insertBefore(RegionUtil.getRegionByElem(braceObj[0]),braceObj[0],fileItemHtml);


			var fileView = fileUploader.wrapperObj.find(".uploaded-files>.file-view:last");
			fileView.children(".delete-btn").click(function(event){
				fileUploader.deleteFile(this);
			});

			fileView.children(".file-info").click(function(event){
				var paraMap = new HashMap();
				paraMap.put("url",this.getAttribute("file-url"));
				
				if(AndroidClient.isFromWebview){//app
					RegionUtil.gotoScreen("videoPreview",paraMap);
				}
				else{
					window.open(this.getAttribute("file-url"),"_blank");
					//var opennedWindow = RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/video-preview.rs",paraMap), "视频预览",2,0.7,0.7,null,null,false,true);
				}
			});

			var limitSize = wrappedObj.attr("limit-size");
			if(limitSize==null)limitSize=1;
			else limitSize = parseInt(limitSize);

			var uploadedCount = fileUploader.wrapperObj.find(".uploaded-files").children().length-1;
			if(uploadedCount>=limitSize){
				//已经达到允许上传的最大个数
				fileUploader.wrapperObj.find(".ctrols>.attach-btn").addClass("hidden");
			}
			else{
				fileUploader.wrapperObj.find(".ctrols>.attach-btn").removeClass("hidden");
			}

		}

		fileUploader.wrapperObj.find(".progress-block").text("");

		if (!!this.onUploaded) {
			eval(this.onUploaded+'.call(this,fileIds)');
		}
		// if(typeof this.onUploaded ==="function"){
		// 	this.onUploaded(fileIds);
		// }
	}

	fileUploader.fileObj[0].value='';
}


FileUploader.prototype.uploadFileToCloud = function(fileKeyPrefix,upToken,md5,fileType){
	var fileUploader = this;
	var file = fileUploader.targetFileBlob;

	var completeDef = new jQuery.Deferred();
	var completePromise = completeDef.promise();
	completePromise.done(function(result){//上传成功,失败或者中止的回调函数
		//CoverPlugin.loadingComplete();
		fileUploader.uploadComplete(result);
	});

	this.observer = {
	  next:function(res){
		  fileUploader.wrapperObj.find(".progress-block").text(Math.round(res.total.percent)+"%");
	  },
	  error:function(err){
		  RegionUtil.error(err);
		  //CoverPlugin.loadingComplete();
		  completeDef.resolve(2);
	  },
	  complete:function(res){
		  RegionUtil.debug("complete");
		  //方案1,主动回调服务器,确认上传
		  fileUploader.confirmCloudFileUploaded(md5,res.key,fileType,completeDef);
	  }
	};

	var observable = qiniu.upload(file,fileKeyPrefix+file.name, upToken, null, null);

	var cancelBtn = fileUploader.wrapperObj.find(".cancel-btn");
	cancelBtn.removeClass("hidden");
	cancelBtn.unbind("click");
	cancelBtn.click(function(){
		fileUploader.subscription.unsubscribe();
		completeDef.resolve(3);//取消
	});
	this.subscription = observable.subscribe(this.observer);
}

/**
 * cloud 上传才需要回调的函数
 */
FileUploader.prototype.confirmCloudFileUploaded=function(md5,fileKey,fileType,jqueryDef){
	var fileUploader = this;
	var reqObj = new Object();
	reqObj.md5 = md5;
	reqObj.fileKey = fileKey;
	RegionUtil.ajaxJsonTask(fileUploader.svcUrl+"/objectsystem/confirm-upload","POST",reqObj,function(serverData,dataPara){
		var responseData = serverData.data;
		jqueryDef.resolve(responseData);
		if(window.filesInfo ==null){
			window.filesInfo = {};
		}
		window.filesInfo[serverData.data.fileId]=serverData.data;
	});
}

/**
 * local upload begin
 */
FileUploader.prototype.uploadFileToLocal = function(fileKeyPrefix,md5,fileType){
	var fileUploader = this;
	var file = fileUploader.targetFileBlob;

	var completeDef = new jQuery.Deferred();
	var completePromise = completeDef.promise();
	completePromise.done(function(result){//上传成功,失败或者中止的回调函数
		fileUploader.uploadComplete(result);
	});

	var formData = new FormData();
    formData.append("upfile" , file);
    formData.append("md5" , md5);
    formData.append("fileType" , fileType);

    this.processingFileReq = $.ajax({
           type: "POST",
           url:fileUploader.svcUrl+"/objectsystem/local-upload",
           data: formData ,
           processData : false,
           contentType : false ,
           xhr: function(){
                var xhr = $.ajaxSettings.xhr();
                if(xhr.upload) {
                    xhr.upload.addEventListener("progress" , function(evt){
                    	var tmpElem = RegionUtil.getEventTarget(evt);
                        var loaded = evt.loaded;
                        var tot = evt.total;
                        var per = Math.floor(100*loaded/tot);
                        fileUploader.wrapperObj.find(".progress-block").text(Math.round(per)+"%");
                    }, false);
                    return xhr;
                }
            },
            success: function (serverResponse) {
            	console.log(serverResponse);
            	if(serverResponse.success==true){
            		completeDef.resolve(serverResponse.data[0]);
            	}
            },
            error : function(XMLHttpRequest, textStatus, errorThrown) {
				if(XMLHttpRequest.readyState==0){
				}
				else{
					completeDef.resolve(2);
				}
            }
        });

	var cancelBtn = fileUploader.wrapperObj.find(".cancel-btn");
	cancelBtn.removeClass("hidden");
	cancelBtn.unbind("click");
	cancelBtn.click(function(){
		fileUploader.processingFileReq.abort();
		completeDef.resolve(3);//取消
	});
}


/**
 * local upload end
 */


/**
 * 验证文件类型 和 大小
 */
FileUploader.prototype.validation = function(event){
	if(this.fileObj[0].files[0]==undefined)return false;//如果没有选中文件

	var maxsize = this.parasMap.get("maxsize");
	var accept = this.parasMap.get("accept");

	if(maxsize==null){
		maxsize=20000;
	}
	else{
		maxsize = parseInt(maxsize);
	}
	if(maxsize>0){
		if(this.fileObj[0].files[0].size > maxsize * 1000) {
			RegionUtil.alert(global_msg.file_size_limit+maxsize+"KB");
			return false;
		}
	}
	if(accept==null||accept=="")accept="*";
	if(accept!="*"){
		var fileExts = accept.split(",");
		var x = 0, fileExtFound = false;
		for(x = 0; x< fileExts.length; x++){
			if(this.fileObj[0].files[0].name.toLowerCase().endWith(fileExts[x].toLowerCase())){
				fileExtFound = true;
				break;
			}
		}
		if(fileExtFound == false) {
			RegionUtil.alert(global_msg.file_type_limit+",支持的类型有 :"+accept);
			return false;
		}
	}
	return true;
}


/**
 * 4 component : EventBus
 *
 */
function EventBus(){
	this.eventHub = {};

	this.eventCodeRegisterByRegionId = {};//每个regionId关注的eventcode数组
}

EventBus.prototype.newListener = function(func){
	var listener = {};
	listener.handle = func;
	return listener;
}

/**
 * 把listener注册到监听管理器以及监听eventCode
 */
EventBus.prototype.addEventListener = function(eventCode,listener){
	if(listener.id==null){
		listener.id = RegionUtil.UUID();
	}

	var eventListeners = this.eventHub[eventCode];
	if(eventListeners==null){
		eventListeners = {};
		this.eventHub[eventCode] = eventListeners;
	}
	eventListeners[listener.id] = listener;

	if(listener.regionId!=null){
		var interestedEventCodeMap = this.eventCodeRegisterByRegionId[listener.regionId];
		if(interestedEventCodeMap==null){
			interestedEventCodeMap = {};
			this.eventCodeRegisterByRegionId[listener.regionId] = interestedEventCodeMap;
		}
		interestedEventCodeMap[eventCode] = true;
	}
}
/**
 * 移除region里所有的listeners
 */
EventBus.prototype.removeListenersForRegion = function(region){
	var interestedEventCodeMap = this.eventCodeRegisterByRegionId[region.regionId];
	for(eventCode in interestedEventCodeMap){
		var eventListeners = this.eventHub[eventCode];
		if(eventListeners!=null){
			for(listenerId in eventListeners){
				var listener = eventListeners[listenerId];
				if(listener.regionId == region.regionId){
					delete eventListeners[listenerId];
				}
			}
		}
	}
}

/**
 * 移除eventCode 对应的listener
 */
EventBus.prototype.removeListener = function(eventCode,listener){
	var eventListeners = this.eventHub[eventCode];
	if(eventListeners!=null){
		delete eventListeners[listener.id];
	}
}

/**
 * eventObject 有两个属性 一个是data,一个是eventCode
 */
EventBus.prototype.publishEvent = function(eventObject){
	// debugger
	var eventListeners = this.eventHub[eventObject.eventCode];
	if(eventListeners!=null){
		for(listenerId in eventListeners){
			try{
				eventListeners[listenerId].handle(eventObject.eventData);
			}
			catch(e){
				console.log(e);
			}

		}
	}
}











