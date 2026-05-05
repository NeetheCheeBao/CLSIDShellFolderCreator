import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import ctypes

try:
    ctypes.windll.shcore.SetProcessDpiAwareness(1)
except Exception:
    pass

class ToolTip:
    def __init__(self, widget, text):
        self.widget = widget
        self.text = text
        self.tipwindow = None
        self.widget.bind("<Enter>", self.show_tip, add="+")
        self.widget.bind("<Leave>", self.hide_tip, add="+")
        self.widget.bind("<Motion>", self.move_tip, add="+")

    def show_tip(self, event):
        if self.tipwindow or not self.text:
            return
        self.tipwindow = tw = tk.Toplevel(self.widget)
        tw.wm_overrideredirect(True)
        tw.wm_attributes("-topmost", True)
        label = tk.Label(tw, text=self.text, background="#333333", foreground="#ffffff", 
                         padx=8, pady=4, relief="flat", font=("Microsoft YaHei", 9))
        label.pack()
        self.move_tip(event)

    def move_tip(self, event):
        if not self.tipwindow:
            return
        x = event.x_root
        y = event.y_root
        tw = self.tipwindow
        tip_width = tw.winfo_reqwidth()
        tip_height = tw.winfo_reqheight()
        screen_width = self.widget.winfo_screenwidth()
        screen_height = self.widget.winfo_screenheight()
        
        cx = x + 15
        cy = y + 15
        
        if cx + tip_width > screen_width:
            cx = x - tip_width - 5
        if cy + tip_height > screen_height:
            cy = y - tip_height - 5
            
        tw.wm_geometry(f"+{cx}+{cy}")

    def hide_tip(self, event):
        if self.tipwindow:
            self.tipwindow.destroy()
        self.tipwindow = None

class App(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("CLSIDShellFolderCreator_1.0.0")
        self.geometry("440x770")
        self.configure(bg="#FFFFFF")
        
        self.output_dir = os.path.join(os.getcwd(), "CreatorOutput")
        self.resize_timer = None
        self.log_counter = 1
        self.is_creating = False
        
        self.guids = {
            ".{ED7BA470-8E54-465E-825C-99712043E01C}": "上帝模式",
            ".{D20EA4E1-3957-11d2-A40B-0C5020524153}": "管理工具",
            ".{7007ACC7-3202-11D1-AAD2-00805FC1270E}": "网络连接",
            ".{60632754-c523-4b62-b45c-4172da012619}": "用户账户",
            ".{36eef7db-88ad-4e81-ad49-0e313f0c35f8}": "Windows 更新",
            ".{450d8fba-ad25-11d0-98a8-0800361b1103}": "文档",
            ".{374DE290-123F-4565-9164-39C4925E467B}": "下载",
            ".{1CF1260C-4DD0-4ebb-811F-33C572699FDE}": "音乐",
            ".{33E28130-4E1E-4676-835A-98395C3BC3BB}": "图片",
            ".{A0953C92-50DC-43bf-BE83-3742FED03C9C}": "视频",
            ".{BD84B380-8CA2-1069-AB1D-08000948534C}": "字体",
            ".{BB06C0E4-D293-4f75-8A90-CB05B6477EEE}": "系统",
            ".{00021400-0000-0000-C000-000000000046}": "桌面",
            ".{031E4825-7B94-4dc3-B131-E946B44C8DD5}": "库",
            ".{208D2C60-3AEA-1069-A2D7-08002B30309D}": "网络",
            ".{20D04FE0-3AEA-1069-A2D8-08002B30309D}": "此电脑",
            ".{21EC2020-3AEA-1069-A2DD-08002B30309D}": "控制面板",
            ".{2227A280-3AEA-1069-A2DE-08002B30309D}": "打印机",
            ".{645FF040-5081-101B-9F08-00AA002F954E}": "回收站",
            ".{8E908FC9-BECC-40f6-915B-F4CA0E70D03D}": "网络和共享中心",
            ".{00020D75-0000-0000-C000-000000000046}": "UNNAM",
            ".{025A5937-A6BE-4686-A844-36FE4BEC8B6D}": "UNNAM",
            ".{04731B67-D933-450a-90E6-4ACD2E9408FE}": "UNNAM",
            ".{05d7b0f4-2121-4eff-bf6b-ed3f69b894d9}": "UNNAM",
            ".{088e3905-0323-4b02-9826-5d99428e115f}": "UNNAM",
            ".{0907616E-F5E6-48D8-9D61-A91C3D28106D}": "UNNAM",
            ".{0AFACED1-E828-11D1-9187-B532F1E9575D}": "UNNAM",
            ".{0c39a5cf-1a7a-40c8-ba74-8900e6df5fcd}": "UNNAM",
            ".{0CD7A5C0-9F37-11CE-AE65-08002B2E1262}": "UNNAM",
            ".{0DB7E03F-FC29-4DC6-9020-FF41B59E513A}": "UNNAM",
            ".{0DF44EAA-FF21-4412-828E-260A8728E7F1}": "UNNAM",
            ".{0E5AAE11-A475-4c5b-AB00-C66DE400274E}": "UNNAM",
            ".{1206F5F1-0569-412C-8FEC-3204630DFB70}": "UNNAM",
            ".{15eae92e-f17a-4431-9f28-805e482dafd4}": "UNNAM",
            ".{17cd9488-1228-4b2f-88ce-4298e93e0966}": "UNNAM",
            ".{1bef2128-2f96-4500-ba7c-098dc0049cb2}": "UNNAM",
            ".{1D2680C9-0E2A-469d-B787-065558BC7D43}": "UNNAM",
            ".{1f3427c8-5c10-4210-aa03-2ee45287d668}": "UNNAM",
            ".{1FA9085F-25A2-489B-85D4-86326EEDCD87}": "UNNAM",
            ".{22877a6d-37a1-461a-91b0-dbda5aaebc99}": "UNNAM",
            ".{241D7C96-F8BF-4F85-B01F-E2B043341A4B}": "UNNAM",
            ".{24ad3ad4-a569-4530-98e1-ab02f9417aa8}": "UNNAM",
            ".{2559a1f0-21d7-11d4-bdaf-00c04f60b9f0}": "UNNAM",
            ".{2559a1f2-21d7-11d4-bdaf-00c04f60b9f0}": "UNNAM",
            ".{2559a1f3-21d7-11d4-bdaf-00c04f60b9f0}": "UNNAM",
            ".{2559a1f5-21d7-11d4-bdaf-00c04f60b9f0}": "UNNAM",
            ".{2559a1f7-21d7-11d4-bdaf-00c04f60b9f0}": "UNNAM",
            ".{2559a1f8-21d7-11d4-bdaf-00c04f60b9f0}": "UNNAM",
            ".{267cf8a9-f4e3-41e6-95b1-af881be130ff}": "UNNAM",
            ".{26EE0668-A00A-44D7-9371-BEB064C98683}": "UNNAM",
            ".{28803F59-3A75-4058-995F-4EE5503B023C}": "UNNAM",
            ".{289978AC-A101-4341-A817-21EBA7FD046D}": "UNNAM",
            ".{289AF617-1CC3-42A6-926C-E6A863F0E3BA}": "UNNAM",
            ".{2965e715-eb66-4719-b53f-1672673bbefa}": "UNNAM",
            ".{2E9E59C0-B437-4981-A647-9C34B9B90891}": "UNNAM",
            ".{2F6CE85C-F9EE-43CA-90C7-8A9BD53A2467}": "UNNAM",
            ".{3080F90D-D7AD-11D9-BD98-0000947B0257}": "UNNAM",
            ".{3080F90E-D7AD-11D9-BD98-0000947B0257}": "UNNAM",
            ".{3134ef9c-6b18-4996-ad04-ed5912e00eb5}": "UNNAM",
            ".{323CA680-C24D-4099-B94D-446DD2D7249E}": "UNNAM",
            ".{328B0346-7EAF-4BBE-A479-7CB88A095F5B}": "UNNAM",
            ".{35786D3C-B075-49b9-88DD-029876E11C01}": "UNNAM",
            ".{38A98528-6CBF-4CA9-8DC0-B1E1D10F7B1B}": "UNNAM",
            ".{3936E9E4-D92C-4EEE-A85A-BC16D5EA0819}": "UNNAM",
            ".{3ADD1653-EB32-4cb0-BBD7-DFA0ABB5ACCA}": "UNNAM",
            ".{3dfdf296-dbec-4fb4-81d1-6a3438bcf4de}": "UNNAM",
            ".{3f6bc534-dfa1-4ab4-ae54-ef25a74e0107}": "UNNAM",
            ".{4026492F-2F69-46B8-B9BF-5654FC07E423}": "UNNAM",
            ".{418c8b64-5463-461d-88e0-75e2afa3c6fa}": "UNNAM",
            ".{4234d49b-0245-4df3-b780-3893943456e1}": "UNNAM",
            ".{4336a54d-038b-4685-ab02-99bb52d3fb8b}": "UNNAM",
            ".{437ff9c0-a07f-4fa0-af80-84b6c6440a16}": "UNNAM",
            ".{4564b25e-30cd-4787-82ba-39e73a750b14}": "UNNAM",
            ".{48e7caab-b918-4e58-a94d-505519c795dc}": "UNNAM",
            ".{5399E694-6CE5-4D6C-8FCE-1D8870FDCBA0}": "UNNAM",
            ".{58E3C745-D971-4081-9034-86E34B30836A}": "UNNAM",
            ".{59031a47-3f72-44a7-89c5-5595fe6b30ee}": "UNNAM",
            ".{5b934b42-522b-4c34-bbfe-37a3ef7b9c90}": "UNNAM",
            ".{5E5F29CE-E0A8-49D3-AF32-7A7BDC173478}": "UNNAM",
            ".{5ea4f148-308c-46d7-98a9-49041b1dd468}": "UNNAM",
            ".{5ED4F38C-D3FF-4D61-B506-6820320AEBFE}": "UNNAM",
            ".{63da6ec0-2e98-11cf-8d82-444553540000}": "UNNAM",
            ".{64693913-1c21-4f30-a98f-4e52906d3b56}": "UNNAM",
            ".{67718415-c450-4f3c-bf8a-b487642dc39b}": "UNNAM",
            ".{6785BFAC-9D2D-4be5-B7E2-59937E8FB80A}": "UNNAM",
            ".{679f85cb-0220-4080-b29b-5540cc05aab6}": "UNNAM",
            ".{67CA7650-96E6-4FDD-BB43-A8E774F73A57}": "UNNAM",
            ".{6DFD7C5C-2451-11d3-A299-00C04F8EF6AF}": "UNNAM",
            ".{71D99464-3B6B-475C-B241-E15883207529}": "UNNAM",
            ".{72b36e70-8700-42d6-a7f7-c9ab3323ee51}": "UNNAM",
            ".{7A9D77BD-5403-11d2-8785-2E0420524153}": "UNNAM",
            ".{7b81be6a-ce2b-4676-a29e-eb907a5126c5}": "UNNAM",
            ".{7BD29E00-76C1-11CF-9DD0-00A0C9034933}": "UNNAM",
            ".{7BD29E01-76C1-11CF-9DD0-00A0C9034933}": "UNNAM",
            ".{863aa9fd-42df-457b-8e4d-0de1b8015c60}": "UNNAM",
            ".{871C5380-42A0-1069-A2EA-08002B30309D}": "UNNAM",
            ".{87630419-6216-4ff8-a1f0-143562d16d5c}": "UNNAM",
            ".{877ca5ac-cb41-4842-9c69-9136e42d47e2}": "UNNAM",
            ".{88C6C381-2E85-11D0-94DE-444553540000}": "UNNAM",
            ".{896664F7-12E1-490f-8782-C0835AFD98FC}": "UNNAM",
            ".{89D83576-6BD1-4c86-9454-BEB04E94C819}": "UNNAM",
            ".{8FD8B88D-30E1-4F25-AC2B-553D3D65F0EA}": "UNNAM",
            ".{9113A02D-00A3-46B9-BC5F-9C04DADDD5D7}": "UNNAM",
            ".{93412589-74D4-4E4E-AD0E-E0CB621440FD}": "UNNAM",
            ".{9343812e-1c37-4a49-a12e-4b2d810d956b}": "UNNAM",
            ".{98F275B4-4FFF-11E0-89E2-7B86DFD72085}": "UNNAM",
            ".{992CFFA0-F557-101A-88EC-00DD010CCC48}": "UNNAM",
            ".{9a096bb5-9dc3-4d1c-8526-c3cbf991ea4e}": "UNNAM",
            ".{9C60DE1E-E5FC-40f4-A487-460851A8D915}": "UNNAM",
            ".{9C73F5E5-7AE7-4E32-A8E8-8D23B85255BF}": "UNNAM",
            ".{9DB7A13C-F208-4981-8353-73CC61AE2783}": "UNNAM",
            ".{9FE63AFD-59CF-4419-9775-ABCC3849F861}": "UNNAM",
            ".{a00ee528-ebd9-48b8-944a-8942113d46ac}": "UNNAM",
            ".{a3c3d402-e56c-4033-95f7-4885e80b0111}": "UNNAM",
            ".{a5a3563a-5755-4a6f-854e-afa3230b199f}": "UNNAM",
            ".{a6482830-08eb-41e2-84c1-73920c2badb9}": "UNNAM",
            ".{A8A91A66-3A7D-4424-8D24-04E180695C7A}": "UNNAM",
            ".{A8CDFF1C-4878-43be-B5FD-F8091C1C60D0}": "UNNAM",
            ".{AEE2420F-D50E-405C-8784-363C582BF45A}": "UNNAM",
            ".{AFDB1F70-2A4C-11d2-9039-00C04F8EEB3E}": "UNNAM",
            ".{b155bdf8-02f0-451e-9a26-ae317cfd7779}": "UNNAM",
            ".{b2952b16-0e07-4e5a-b993-58c52cb94cae}": "UNNAM",
            ".{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}": "UNNAM",
            ".{B4FB3F98-C1EA-428d-A78A-D1F5659CBA93}": "UNNAM",
            ".{B98A2BEA-7D42-4558-8BD1-832F41BAC6FD}": "UNNAM",
            ".{BB64F8A7-BEE7-4E1A-AB8D-7D8273F7FDB6}": "UNNAM",
            ".{BC48B32F-5910-47F5-8570-5074A8A5636A}": "UNNAM",
            ".{BD7A2E7B-21CB-41b2-A086-B309680C6B7E}": "UNNAM",
            ".{BD84B380-8CA2-1069-AB1D-08000948F534}": "UNNAM",
            ".{C2B136E2-D50E-405C-8784-363C582BF43E}": "UNNAM",
            ".{c57a6066-66a3-4d91-9eb9-41532179f0a5}": "UNNAM",
            ".{C58C4893-3BE0-4B45-ABB5-A63E4B8C8651}": "UNNAM",
            ".{D2035EDF-75CB-4EF1-95A7-410D9EE17170}": "UNNAM",
            ".{d3162b92-9365-467a-956b-92703aca08af}": "UNNAM",
            ".{d34a6ca6-62c2-4c34-8a7c-14709c1ad938}": "UNNAM",
            ".{D4480A50-BA28-11d1-8E75-00C04FA31A86}": "UNNAM",
            ".{d450a8a1-9568-45c7-9c0e-b4f9fb4537bd}": "UNNAM",
            ".{D555645E-D4F8-4c29-A827-D93C859C4F2A}": "UNNAM",
            ".{D9EF8727-CAC2-4e60-809E-86F80A666C91}": "UNNAM",
            ".{daf95313-e44d-46af-be1b-cbacea2c3065}": "UNNAM",
            ".{DFFACDC5-679F-4156-8947-C5C76BC0B67F}": "UNNAM",
            ".{e345f35f-9397-435c-8f95-4e922c26259e}": "UNNAM",
            ".{E413D040-6788-4C22-957E-175D1C513A34}": "UNNAM",
            ".{E7E4BC40-E76A-11CE-A9BB-00AA004AE837}": "UNNAM",
            ".{E88DCCE0-B7B3-11d1-A9F0-00AA0060FA31}": "UNNAM",
            ".{ECDB0924-4208-451E-8EE0-373C0956DE16}": "UNNAM",
            ".{ed50fc29-b964-48a9-afb3-15ebb9b97f36}": "UNNAM",
            ".{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921}": "UNNAM",
            ".{EDC978D6-4D53-4b2f-A265-5805674BE568}": "UNNAM",
            ".{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}": "UNNAM",
            ".{F1390A9A-A3F4-4E5D-9C5F-98F3BD8D935C}": "UNNAM",
            ".{F3364BA0-65B9-11CE-A9BA-00AA004AE837}": "UNNAM",
            ".{F5175861-2688-11d0-9C5E-00AA00A45957}": "UNNAM",
            ".{F5FB2C77-0E2F-4A16-A381-3E560C68BC83}": "UNNAM",
            ".{F6B6E965-E9B2-444B-9286-10C9152EDBC5}": "UNNAM",
            ".{f8278c54-a712-415b-b593-b77a2be0dda9}": "UNNAM",
            ".{f86fa3ab-70d2-4fc7-9c99-fcbf05467f3a}": "UNNAM",
            ".{f8c2ab3b-17bc-41da-9758-339d7dbf2d88}": "UNNAM",
            ".{F942C606-0914-47AB-BE56-1321B8035096}": "UNNAM",
            ".{FF393560-C2A7-11CF-BFF4-444553540000}": "UNNAM"
        }
        
        self.setup_ui()
        self.bind("<F1>", lambda event: self.create_all())
        self.bind("<F2>", lambda event: self.clear_log())
        self.bind("<F3>", lambda event: self.open_url())
        self.bind("<Configure>", self.on_window_resize)

    def _on_mousewheel(self, event):
        self.canvas.yview_scroll(int(-1 * (event.delta / 120)), "units")

    def on_window_resize(self, event):
        if event.widget == self:
            if self.resize_timer:
                self.after_cancel(self.resize_timer)
            self.resize_timer = self.after(200, self.update_layout)

    def update_layout(self):
        self.canvas.itemconfig(self.frame_window, width=self.canvas.winfo_width())
        self.canvas.configure(scrollregion=self.canvas.bbox("all"))

    def setup_ui(self):
        toolbar = tk.Frame(self, bg="#F3F3F3", height=45)
        toolbar.pack(side="top", fill="x")
        toolbar.pack_propagate(False)
        
        self.btn_create = tk.Label(toolbar, text="创建全部 (F1)", bg="#F3F3F3", foreground="#333333",
                             font=("Microsoft YaHei UI", 10), padx=12, cursor="hand2")
        self.btn_create.pack(side="left", fill="y")
        self.btn_create.bind("<Button-1>", lambda e: self.create_all())
        self.btn_create.bind("<Enter>", self.on_create_enter)
        self.btn_create.bind("<Leave>", self.on_create_leave)
        
        self.btn_clear = tk.Label(toolbar, text="清空终端 (F2)", bg="#F3F3F3", foreground="#333333",
                             font=("Microsoft YaHei UI", 10), padx=12, cursor="hand2")
        self.btn_clear.pack(side="left", fill="y")
        self.btn_clear.bind("<Button-1>", lambda e: self.clear_log())
        self.btn_clear.bind("<Enter>", self.on_clear_enter)
        self.btn_clear.bind("<Leave>", self.on_clear_leave)

        self.btn_url = tk.Label(toolbar, text="项目地址 (F3)", bg="#F3F3F3", foreground="#333333",
                          font=("Microsoft YaHei UI", 10), padx=12, cursor="hand2")
        self.btn_url.pack(side="left", fill="y")
        self.btn_url.bind("<Button-1>", lambda e: self.open_url())
        self.btn_url.bind("<Enter>", lambda e, w=self.btn_url: w.config(bg="#E5E5E5"))
        self.btn_url.bind("<Leave>", lambda e, w=self.btn_url: w.config(bg="#F3F3F3"))

        log_frame = tk.Frame(self, bg="#FFFFFF")
        log_frame.pack(side="top", fill="x", padx=15, pady=10)
        log_frame.grid_columnconfigure(0, weight=1)
        log_frame.grid_rowconfigure(0, weight=1)
        
        log_scroll_y = ttk.Scrollbar(log_frame, orient="vertical")
        log_scroll_x = ttk.Scrollbar(log_frame, orient="horizontal")
        
        self.log_box = tk.Text(log_frame, height=8, font=("Consolas", 10), bg="#F9F9F9",
                              relief="flat", highlightthickness=1, highlightbackground="#E0E0E0",
                              yscrollcommand=log_scroll_y.set, xscrollcommand=log_scroll_x.set, 
                              wrap="none")
        
        self.log_box.grid(row=0, column=0, sticky="nsew")
        log_scroll_y.grid(row=0, column=1, sticky="ns")
        log_scroll_x.grid(row=1, column=0, sticky="ew")
        
        log_scroll_y.config(command=self.log_box.yview)
        log_scroll_x.config(command=self.log_box.xview)
        
        self.log_box.tag_config("green", foreground="#28a745")
        self.log_box.tag_config("red", foreground="#dc3545")
        self.log_box.config(state="disabled")
        
        container = tk.Frame(self, bg="#FFFFFF")
        container.pack(side="top", fill="both", expand=True, padx=15, pady=5)
        
        self.canvas = tk.Canvas(container, bg="#FFFFFF", highlightthickness=0)
        scrollbar = ttk.Scrollbar(container, orient="vertical", command=self.canvas.yview)
        self.scrollable_frame = tk.Frame(self.canvas, bg="#FFFFFF")
        
        self.frame_window = self.canvas.create_window((0, 0), window=self.scrollable_frame, anchor="nw")
        self.canvas.configure(yscrollcommand=scrollbar.set)
        self.canvas.pack(side="left", fill="both", expand=True)
        scrollbar.pack(side="right", fill="y")
        
        self.canvas.bind("<MouseWheel>", self._on_mousewheel)
        self.scrollable_frame.bind("<MouseWheel>", self._on_mousewheel)

        for index, (guid, name) in enumerate(self.guids.items(), 1):
            row_frame = tk.Frame(self.scrollable_frame, bg="#FFFFFF")
            row_frame.pack(side="top", pady=6)
            
            lbl_index = tk.Label(row_frame, text=f"({index})", font=("Microsoft YaHei UI", 10), 
                                 bg="#FFFFFF", fg="#999999", width=4, anchor="e")
            lbl_index.pack(side="left", padx=(0, 5))
            
            btn = tk.Button(row_frame, text=name, width=28, font=("Microsoft YaHei UI", 10), 
                            bg="#F3F3F3", foreground="#333333", activebackground="#DCDCDC",
                            relief="flat", bd=0, pady=8, cursor="hand2")
            btn.config(command=lambda g=guid, n=name: self.create_folder(g, n))
            btn.pack(side="left")
            
            btn.bind("<Enter>", lambda e, w=btn: w.config(bg="#E5E5E5"), add="+")
            btn.bind("<Leave>", lambda e, w=btn: w.config(bg="#F3F3F3"), add="+")
            
            row_frame.bind("<MouseWheel>", self._on_mousewheel)
            lbl_index.bind("<MouseWheel>", self._on_mousewheel)
            btn.bind("<MouseWheel>", self._on_mousewheel)
            
            ToolTip(btn, guid)

    def on_create_enter(self, event):
        if not self.is_creating:
            self.btn_create.config(bg="#E5E5E5")

    def on_create_leave(self, event):
        if not self.is_creating:
            self.btn_create.config(bg="#F3F3F3")

    def on_clear_enter(self, event):
        if not self.is_creating:
            self.btn_clear.config(bg="#E5E5E5")

    def on_clear_leave(self, event):
        if not self.is_creating:
            self.btn_clear.config(bg="#F3F3F3")

    def clear_log(self):
        if self.is_creating:
            return
        self.log_box.config(state="normal")
        self.log_box.delete("1.0", "end")
        self.log_box.config(state="disabled")
        self.update_idletasks()

    def log_message(self, message, tag):
        self.log_box.config(state="normal")
        formatted_message = f"[{self.log_counter}] {message}"
        self.log_box.insert("end", formatted_message + "\n", tag)
        self.log_counter += 1
        
        lines = int(self.log_box.index('end-1c').split('.')[0])
        if lines > 200:
            self.log_box.delete("1.0", f"{lines-200}.0")
            
        self.log_box.see("end")
        self.log_box.config(state="disabled")
        self.update_idletasks()

    def create_folder(self, guid, name):
        if not os.path.exists(self.output_dir):
            try:
                os.makedirs(self.output_dir)
            except: pass
        target_path = os.path.join(self.output_dir, guid)
        try:
            if not os.path.exists(target_path):
                os.mkdir(target_path)
                self.log_message(f"{name}创建成功，GUID为{guid}，路径为{target_path}", "green")
            else:
                self.log_message(f"{name}创建失败，GUID为{guid}，已存在于路径{target_path}", "red")
        except Exception:
            self.log_message(f"{name}创建失败，GUID为{guid}，已存在于路径{target_path}", "red")

    def create_all(self):
        if self.is_creating:
            return
        self.is_creating = True
        self.btn_create.config(foreground="#999999", bg="#F3F3F3", cursor="arrow")
        self.btn_clear.config(foreground="#999999", bg="#F3F3F3", cursor="arrow")
        self.update()
        
        for guid, name in self.guids.items():
            self.create_folder(guid, name)
            self.update()
            
        if self.winfo_exists():
            self.after(1000, self.reset_create_button)

    def reset_create_button(self):
        self.is_creating = False
        self.btn_create.config(foreground="#333333", cursor="hand2")
        self.btn_clear.config(foreground="#333333", cursor="hand2")

    def open_url(self):
        webbrowser.open("https://github.com/NeetheCheeBao/CLSIDShellFolderCreator")

if __name__ == "__main__":
    app = App()
    app.mainloop()