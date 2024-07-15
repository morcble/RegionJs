
CKEDITOR.dialog.add('latex', function (editor) { //要和plugin.js 中的command 一致
    var escape = function (value) {
        return value;
    };
    
    var numbering = function( id ) {
		return CKEDITOR.tools.getNextId() + '_' + id;
	},
	previewImageId = numbering( 'previewImage' );
	latexInputId = numbering( 'latexInput' );
	templateOptionId = numbering( 'templateOption' );
	
	var previewed = false;
    return {
        title: '插入latex排版', //对话框标题
        minWidth: 800, //对话框宽度
        minHeight: 450,//对话框高度
        contents: [{   //对话框内容
            id: 'tableLayoutId',
            elements: [
            {
            	id: 'latexIntro',
                type: 'html',
				style: 'width:100%;',
				html: 'Latex表达式将会被转换为图片<div style="float:right"><select id="'+templateOptionId+'" style="height:20px;border:1px solid #000"><option value="1">使用简单模板</option><option value="0">不使用模板</option><select></div>'
            },
            {
                id: 'latexInput',
                type: 'html',
				style: 'width:100%;border:1px solid #000',
				html: '<textarea rows="12" cols=""  class="form-control" maxlength="2000" id="'+latexInputId+'"></textarea>'  
            },
            {
                id: 'previewBtn',
                type: 'button',
                label:'编译Latex表达式',
                style: 'margin-left: 40%;width:20%;',
                onClick: function() {
                	var latexInputArea = CKEDITOR.document.getById( latexInputId );
                	var expression = latexInputArea.getValue().trim();
                	latexInputArea.setValue(expression);
                	
                	var templateVal = CKEDITOR.document.getById( templateOptionId ).getValue();
                	
                	if(expression.trim()==""){
                		alert("Latex 表达式为空");
                	}
                	else{
                		CoverPlugin.loadingStart();
                		var jqueryDef = new jQuery.Deferred();
                		var promise = jqueryDef.promise();
                		
                		var reqObj = new Object();
            			reqObj.templateVal = templateVal;
            			reqObj.expression = expression;
            			RegionUtil.ajaxJsonTask(Config.backendPath+"/latex/compile-and-upload","POST",reqObj,function(serverData,dataPara){
            				jqueryDef.resolve(serverData);
            			});
                		
            			
            			promise.done(function(rsData){
            				if(rsData.data.success){
            					var imgElem = CKEDITOR.document.getById(previewImageId);
                				imgElem.setAttribute("src",rsData.data.url);
                				imgElem.setAttribute("alt",rsData.data.fileId);
            				}
            				else{
            					RegionUtil.debug(rsData);
            					RegionUtil.alert("编译错误, step="+rsData.data.step+", error Detail= "+rsData.data.stepLog);
            				}
            				previewed = true;
            				CoverPlugin.loadingComplete();
            			});
                	}
                	
                }
            },
            {  
                id: 'previewImageArea',
                type: 'html',
				style: 'width:100%;',
				html: '<div style="margin:0 auto;text-align:center;min-height:150px;overflow:scroll;width:800px;"><p style="margin:0;vertical-align: middle"><img id="' + previewImageId + '" alt="" /></p></div>'  
            }
            ]
        }],
        onShow: function() {
        	try{
        		var imgElem = CKEDITOR.document.getById(previewImageId);
        		var tdElem = imgElem.$.parentElement.parentElement.parentElement;
        		tdElem.style=tdElem.style+";background:#f1ecec;"
        	}
        	catch(e){
        		console.log(e)
        	}
        },
        onOk: function () { 
        	if(!previewed){
        		RegionUtil.alert("请先编译Latex表达式");
        		return false;
        	}
        	
        	var srcImgElem = CKEDITOR.document.getById(previewImageId);
        	if(srcImgElem.getAttribute("alt")!=null){
        		var imageElement = editor.document.createElement( 'img' );
            	imageElement.setAttribute("alt",srcImgElem.getAttribute("alt"));
            	imageElement.setAttribute("src",srcImgElem.getAttribute("src"));
            	editor.insertElement(imageElement);
            	
//            	//clear
//            	srcImgElem.removeAttribute("alt");
//            	srcImgElem.removeAttribute("src");
//            	var latexInputArea = CKEDITOR.document.getById( latexInputId );
//            	latexInputArea.setValue("");
            	
            	previewed = false;
            	return true;
        	}
            else {
                return false;
            }
        }
    };
});
