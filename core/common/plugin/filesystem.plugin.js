/**
 * 调用文件上传  移除或者增加本地上传功能
 */
var ObjectSystemUtil = {
		uploadFile:function(file,fileType,vendor,storePath){
			ObjectSystemUtil.file = file;
			
			if(fileType==null)fileType="common";
			
			var completeDef = new jQuery.Deferred();
			var completePromise = completeDef.promise();
			if(vendor==null)vendor = Config.fileVendor;
			if(vendor==null)vendor="cloud";//默认为云上传
			
			if(storePath!=null && !storePath.endsWith("/"))storePath=storePath+"/";
			
			var jqueryDef = RegionUtil.calculateMD5(file);
			jqueryDef.done(function(md5){
				var reqObj = new Object();
				reqObj.md5 = md5;
				reqObj.upToken = RegionUtil.getCookie("fileUpToken");
				reqObj.fileType = fileType;
				reqObj.vendor = vendor;
				reqObj.storePath = storePath;
				//CoverPlugin.loadingStart();
				RegionUtil.ajaxJsonTask(Config.backendPath+"/RCS_Objectsystem/objectsystem/pre-upload","POST",reqObj,function(serverData,dataPara){
					var responseData = serverData.data;
					
					if(!responseData.recordExsit){
						//if(reqObj.upToken == null&&responseData.upToken!=null){
						if(responseData.upToken!=null){
							RegionUtil.setCookieWithExpire("fileUpToken",responseData.upToken,"s"+60)
							reqObj.upToken = responseData.upToken;
						}
						
						if(vendor=="local"){
							var localPromise = ObjectSystemUtil.subUploadFileToLocal(responseData.fileKeyPrefix,reqObj.md5,reqObj.fileType);
							localPromise.done(function(serverData){
								debugger
								completeDef.resolve(serverData);
							})
							return;
						}
						
						
						
						ObjectSystemUtil.observer = {
						  next:function(res){
							  RegionUtil.debug(res);
						  },
						  error:function(err){
							  RegionUtil.error(err);
							  ObjectSystemUtil.subscription = null;
							  ObjectSystemUtil.observer = null;
							  ObjectSystemUtil.file = null;
							  //CoverPlugin.loadingComplete();
							  completeDef.resolve(2);
						  }, 
						  complete:function(res){
							  RegionUtil.debug(res);
							  RegionUtil.debug("complete");
							  //方案1,主动回调服务器,确认上传
							  completePromise.done(function(){
								  ObjectSystemUtil.subscription = null;
								  ObjectSystemUtil.observer = null;
								  ObjectSystemUtil.file = null;
							  });
							  ObjectSystemUtil.confirmFileUploaded(reqObj.md5,res.key,reqObj.fileType,completeDef);  
						  }
						};
						
						var observable = qiniu.upload(ObjectSystemUtil.file, responseData.fileKeyPrefix+ObjectSystemUtil.file.name, reqObj.upToken, null, null);
						ObjectSystemUtil.subscription = observable.subscribe(ObjectSystemUtil.observer);
					}
					else{//找到重复的文件 
						//donothing
						//CoverPlugin.loadingComplete();
						completeDef.resolve(responseData);
					}
				})
				
			})
			
			return completePromise;
		},
		confirmFileUploaded:function(md5,fileKey,fileType,jqueryDef){
			var reqObj = new Object();
			reqObj.md5 = md5;
			reqObj.fileKey = fileKey;
			reqObj.fileType = fileType;
			RegionUtil.ajaxJsonTask(Config.backendPath+"/RCS_Objectsystem/objectsystem/confirm-upload","POST",reqObj,function(serverData,dataPara){
				var responseData = serverData.data;
				//RegionUtil.debug(responseData);
				jqueryDef.resolve(responseData);
				if(window.filesInfo ==null){
					window.filesInfo = {};
				}
				window.filesInfo[serverData.data.fileId]=serverData.data;
			});
		},
		subUploadFileToLocal : function(fileKeyPrefix,md5,fileType){
			var file = ObjectSystemUtil.file;

			var completeDef = new jQuery.Deferred();
			var completePromise = completeDef.promise();

			var formData = new FormData();
		    formData.append("upfile" , file);
		    formData.append("md5" , md5);
		    formData.append("fileType" , fileType);

		    this.processingFileReq = $.ajax({
		           type: "POST",
		           url:Config.backendPath+"/RCS_Objectsystem/objectsystem/local-upload",
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
		                    }, false);
		                    return xhr;
		                }
		            },
		            success: function (serverResponse) {
		            	console.log(serverResponse);
		            	ObjectSystemUtil.file = null;
		            	if(serverResponse.success==true){
		            		completeDef.resolve(serverResponse.data[0]);
		            	}
		            },
		            error : function(XMLHttpRequest, textStatus, errorThrown) {
		            	ObjectSystemUtil.file = null;
						if(XMLHttpRequest.readyState==0){
						}
						else{
							completeDef.resolve(2);
						}
		            }
		        });
		    return completePromise;
		}
}
