(function(){ 
    CKEDITOR.plugins.add('video',{ 
    	requires: ["dialog"],
        init:function(editor){ 
            editor.addCommand('video', new CKEDITOR.dialogCommand('video'));
            editor.ui.addButton('video',{ 
                label:'插入视频', 
                icon: this.path + 'logo_video.png', 
                command:'video',
                click:function(editor){
                	var paraMap = new HashMap();
                	paraMap.put("ckEditorName",editor.name);
        			var result = RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl('common/shared/preview/ck_videoupload.rs',paraMap),"上传视频",2,null,null,500,350);	
        			
        			/*result.close.done(function(){
        				
    				});*/	
                }
            }); 
            
            //CKEDITOR.dialog.add("video", this.path + "dialogs/video.js",0)
        }
    }); 
})(); 