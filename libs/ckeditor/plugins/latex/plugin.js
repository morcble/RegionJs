(function(){ 
    CKEDITOR.plugins.add('latex',{ 
    	requires: ["dialog"],
        init:function(editor){ 
            editor.addCommand('latex', new CKEDITOR.dialogCommand('latex'));
            editor.ui.addButton('latex',{ 
                label:'latex表达式', 
                icon: this.path + 'logo_latex.png', 
                command:'latex' 
            }); 
            
            CKEDITOR.dialog.add("latex", this.path + "dialogs/latex.js")
        } 
    }); 
})(); 