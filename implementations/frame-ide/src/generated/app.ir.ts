import { defineFrameIrDocument } from '@frame/runtime-dom';

const ir = defineFrameIrDocument({
  "version": 1,
  "components": [
    {
      "name": "IdeApp",
      "props": [],
      "state": [
        {
          "name": "files",
          "value_type": "List",
          "default": "List",
          "source": {
            "start": 44,
            "end": 63
          }
        },
        {
          "name": "currentPath",
          "value_type": "Text",
          "default": {
            "Text": ""
          },
          "source": {
            "start": 64,
            "end": 89
          }
        },
        {
          "name": "currentContent",
          "value_type": "Text",
          "default": {
            "Text": ""
          },
          "source": {
            "start": 90,
            "end": 118
          }
        },
        {
          "name": "status",
          "value_type": "Text",
          "default": {
            "Text": "Frame IDE — Ready"
          },
          "source": {
            "start": 119,
            "end": 158
          }
        },
        {
          "name": "lspConnected",
          "value_type": "Bool",
          "default": {
            "Bool": false
          },
          "source": {
            "start": 159,
            "end": 188
          }
        }
      ],
      "slots": [],
      "nodes": [
        {
          "Element": {
            "kind": "panel",
            "semantic_kind": "panel",
            "render_kind": "div",
            "name": "Root",
            "style": {
              "Automatic": {
                "style": "Root",
                "source": {
                  "start": 212,
                  "end": 216
                }
              }
            },
            "attributes": [],
            "bindings": [],
            "events": [],
            "conditions": [],
            "children": [
              {
                "Element": {
                  "kind": "title",
                  "semantic_kind": "title",
                  "render_kind": "h2",
                  "name": "Title",
                  "style": {
                    "Automatic": {
                      "style": "Title",
                      "source": {
                        "start": 219,
                        "end": 224
                      }
                    }
                  },
                  "attributes": [
                    {
                      "name": "value",
                      "value": {
                        "Literal": "Frame IDE"
                      },
                      "source": {
                        "start": 219,
                        "end": 242
                      }
                    }
                  ],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [],
                  "source": {
                    "start": 219,
                    "end": 242
                  }
                }
              },
              {
                "Element": {
                  "kind": "toolbar",
                  "semantic_kind": "toolbar",
                  "render_kind": "div",
                  "name": "Toolbar",
                  "style": {
                    "Automatic": {
                      "style": "Toolbar",
                      "source": {
                        "start": 257,
                        "end": 264
                      }
                    }
                  },
                  "attributes": [],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [
                    {
                      "Element": {
                        "kind": "action",
                        "semantic_kind": "action",
                        "render_kind": "button",
                        "name": "NewFile",
                        "style": {
                          "Explicit": {
                            "style": "ToolbarButton",
                            "source": {
                              "start": 290,
                              "end": 303
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [
                          {
                            "event": "press",
                            "modifiers": [],
                            "handler": "newFile",
                            "source": {
                              "start": 328,
                              "end": 355
                            }
                          }
                        ],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "label",
                              "semantic_kind": "label",
                              "render_kind": "span",
                              "name": "Label",
                              "style": {
                                "Automatic": {
                                  "style": "Label",
                                  "source": {
                                    "start": 306,
                                    "end": 311
                                  }
                                }
                              },
                              "attributes": [
                                {
                                  "name": "value",
                                  "value": {
                                    "Literal": "New"
                                  },
                                  "source": {
                                    "start": 306,
                                    "end": 327
                                  }
                                }
                              ],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [],
                              "source": {
                                "start": 306,
                                "end": 327
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 267,
                          "end": 365
                        }
                      }
                    },
                    {
                      "Element": {
                        "kind": "action",
                        "semantic_kind": "action",
                        "render_kind": "button",
                        "name": "OpenFile",
                        "style": {
                          "Explicit": {
                            "style": "ToolbarButton",
                            "source": {
                              "start": 390,
                              "end": 403
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [
                          {
                            "event": "press",
                            "modifiers": [],
                            "handler": "openFile",
                            "source": {
                              "start": 429,
                              "end": 457
                            }
                          }
                        ],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "label",
                              "semantic_kind": "label",
                              "render_kind": "span",
                              "name": "Label",
                              "style": {
                                "Automatic": {
                                  "style": "Label",
                                  "source": {
                                    "start": 406,
                                    "end": 411
                                  }
                                }
                              },
                              "attributes": [
                                {
                                  "name": "value",
                                  "value": {
                                    "Literal": "Open"
                                  },
                                  "source": {
                                    "start": 406,
                                    "end": 428
                                  }
                                }
                              ],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [],
                              "source": {
                                "start": 406,
                                "end": 428
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 366,
                          "end": 467
                        }
                      }
                    },
                    {
                      "Element": {
                        "kind": "action",
                        "semantic_kind": "action",
                        "render_kind": "button",
                        "name": "SaveFile",
                        "style": {
                          "Explicit": {
                            "style": "ToolbarButton",
                            "source": {
                              "start": 492,
                              "end": 505
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [
                          {
                            "event": "press",
                            "modifiers": [],
                            "handler": "saveFile",
                            "source": {
                              "start": 531,
                              "end": 559
                            }
                          }
                        ],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "label",
                              "semantic_kind": "label",
                              "render_kind": "span",
                              "name": "Label",
                              "style": {
                                "Automatic": {
                                  "style": "Label",
                                  "source": {
                                    "start": 508,
                                    "end": 513
                                  }
                                }
                              },
                              "attributes": [
                                {
                                  "name": "value",
                                  "value": {
                                    "Literal": "Save"
                                  },
                                  "source": {
                                    "start": 508,
                                    "end": 530
                                  }
                                }
                              ],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [],
                              "source": {
                                "start": 508,
                                "end": 530
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 468,
                          "end": 569
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 243,
                    "end": 577
                  }
                }
              },
              {
                "Element": {
                  "kind": "split",
                  "semantic_kind": "split",
                  "render_kind": "div",
                  "name": "MainSplit",
                  "style": {
                    "Automatic": {
                      "style": "MainSplit",
                      "source": {
                        "start": 590,
                        "end": 599
                      }
                    }
                  },
                  "attributes": [],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [
                    {
                      "Element": {
                        "kind": "panel",
                        "semantic_kind": "panel",
                        "render_kind": "div",
                        "name": "Sidebar",
                        "style": {
                          "Automatic": {
                            "style": "Sidebar",
                            "source": {
                              "start": 616,
                              "end": 623
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "title",
                              "semantic_kind": "title",
                              "render_kind": "h2",
                              "name": "Title",
                              "style": {
                                "Automatic": {
                                  "style": "Title",
                                  "source": {
                                    "start": 626,
                                    "end": 631
                                  }
                                }
                              },
                              "attributes": [
                                {
                                  "name": "value",
                                  "value": {
                                    "Literal": "Explorer"
                                  },
                                  "source": {
                                    "start": 626,
                                    "end": 652
                                  }
                                }
                              ],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [],
                              "source": {
                                "start": 626,
                                "end": 652
                              }
                            }
                          },
                          {
                            "List": {
                              "item": "file",
                              "collection": "files",
                              "key": null,
                              "children": [
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "FileItem",
                                    "style": {
                                      "Automatic": {
                                        "style": "FileItem",
                                        "source": {
                                          "start": 701,
                                          "end": 709
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [
                                      {
                                        "event": "press",
                                        "modifiers": [],
                                        "handler": "openFileByPath",
                                        "source": {
                                          "start": 737,
                                          "end": 775
                                        }
                                      }
                                    ],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Text": {
                                          "value": {
                                            "DataRef": "file"
                                          },
                                          "source": {
                                            "start": 712,
                                            "end": 736
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 684,
                                      "end": 789
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 653,
                                "end": 801
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 602,
                          "end": 811
                        }
                      }
                    },
                    {
                      "Element": {
                        "kind": "panel",
                        "semantic_kind": "panel",
                        "render_kind": "div",
                        "name": "Editor",
                        "style": {
                          "Automatic": {
                            "style": "Editor",
                            "source": {
                              "start": 826,
                              "end": 832
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "editor",
                              "semantic_kind": "editor",
                              "render_kind": "textarea",
                              "name": "CodeEditor",
                              "style": {
                                "Automatic": {
                                  "style": "CodeEditor",
                                  "source": {
                                    "start": 852,
                                    "end": 862
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [
                                {
                                  "property": "value",
                                  "state": "currentContent",
                                  "source": {
                                    "start": 892,
                                    "end": 930
                                  }
                                }
                              ],
                              "events": [
                                {
                                  "event": "input",
                                  "modifiers": [],
                                  "handler": "editorChange",
                                  "source": {
                                    "start": 931,
                                    "end": 965
                                  }
                                },
                                {
                                  "event": "keydown",
                                  "modifiers": [],
                                  "handler": "editorKeydown",
                                  "source": {
                                    "start": 966,
                                    "end": 1003
                                  }
                                }
                              ],
                              "conditions": [],
                              "children": [
                                {
                                  "Element": {
                                    "kind": "label",
                                    "semantic_kind": "label",
                                    "render_kind": "span",
                                    "name": "Label",
                                    "style": {
                                      "Automatic": {
                                        "style": "Label",
                                        "source": {
                                          "start": 865,
                                          "end": 870
                                        }
                                      }
                                    },
                                    "attributes": [
                                      {
                                        "name": "value",
                                        "value": {
                                          "Literal": "Editor"
                                        },
                                        "source": {
                                          "start": 865,
                                          "end": 891
                                        }
                                      }
                                    ],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [],
                                    "source": {
                                      "start": 865,
                                      "end": 891
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 835,
                                "end": 1015
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 812,
                          "end": 1025
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 578,
                    "end": 1033
                  }
                }
              },
              {
                "Element": {
                  "kind": "dock",
                  "semantic_kind": "dock",
                  "render_kind": "div",
                  "name": "StatusBar",
                  "style": {
                    "Automatic": {
                      "style": "StatusBar",
                      "source": {
                        "start": 1045,
                        "end": 1054
                      }
                    }
                  },
                  "attributes": [],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [
                    {
                      "Element": {
                        "kind": "label",
                        "semantic_kind": "label",
                        "render_kind": "span",
                        "name": "Label",
                        "style": {
                          "Automatic": {
                            "style": "Label",
                            "source": {
                              "start": 1057,
                              "end": 1062
                            }
                          }
                        },
                        "attributes": [
                          {
                            "name": "value",
                            "value": {
                              "DataRef": "status"
                            },
                            "source": {
                              "start": 1057,
                              "end": 1078
                            }
                          }
                        ],
                        "bindings": [],
                        "events": [],
                        "conditions": [],
                        "children": [],
                        "source": {
                          "start": 1057,
                          "end": 1078
                        }
                      }
                    },
                    {
                      "Element": {
                        "kind": "action",
                        "semantic_kind": "action",
                        "render_kind": "button",
                        "name": "LspToggle",
                        "style": {
                          "Explicit": {
                            "style": "ToolbarButton",
                            "source": {
                              "start": 1104,
                              "end": 1117
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [
                          {
                            "event": "press",
                            "modifiers": [],
                            "handler": "toggleLsp",
                            "source": {
                              "start": 1142,
                              "end": 1171
                            }
                          }
                        ],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "label",
                              "semantic_kind": "label",
                              "render_kind": "span",
                              "name": "Label",
                              "style": {
                                "Automatic": {
                                  "style": "Label",
                                  "source": {
                                    "start": 1120,
                                    "end": 1125
                                  }
                                }
                              },
                              "attributes": [
                                {
                                  "name": "value",
                                  "value": {
                                    "Literal": "LSP"
                                  },
                                  "source": {
                                    "start": 1120,
                                    "end": 1141
                                  }
                                }
                              ],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [],
                              "source": {
                                "start": 1120,
                                "end": 1141
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 1079,
                          "end": 1181
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 1034,
                    "end": 1189
                  }
                }
              }
            ],
            "source": {
              "start": 202,
              "end": 1195
            }
          }
        }
      ],
      "capabilities": [
        "EventBinding",
        "ListRendering",
        "TwoWayBinding"
      ],
      "source": {
        "start": 15,
        "end": 1201
      }
    }
  ]
} as const);

export default ir;
