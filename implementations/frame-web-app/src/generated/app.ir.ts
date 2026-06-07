import { defineFrameIrDocument } from '@frame/runtime-dom';

const ir = defineFrameIrDocument({
  "version": 1,
  "components": [
    {
      "name": "TodoApp",
      "props": [],
      "state": [
        {
          "name": "items",
          "value_type": "List",
          "default": "List",
          "source": {
            "start": 30,
            "end": 49
          }
        },
        {
          "name": "draft",
          "value_type": "Text",
          "default": {
            "Text": ""
          },
          "source": {
            "start": 50,
            "end": 69
          }
        },
        {
          "name": "nextId",
          "value_type": "Number",
          "default": {
            "Number": "1"
          },
          "source": {
            "start": 70,
            "end": 91
          }
        }
      ],
      "slots": [],
      "nodes": [
        {
          "Element": {
            "kind": "stack",
            "semantic_kind": "stack",
            "render_kind": "div",
            "name": "MainPanel",
            "style": {
              "Automatic": {
                "style": "MainPanel",
                "source": {
                  "start": 116,
                  "end": 125
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
                        "start": 128,
                        "end": 133
                      }
                    }
                  },
                  "attributes": [
                    {
                      "name": "value",
                      "value": {
                        "Literal": "Frame Todo"
                      },
                      "source": {
                        "start": 128,
                        "end": 152
                      }
                    }
                  ],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [],
                  "source": {
                    "start": 128,
                    "end": 152
                  }
                }
              },
              {
                "Element": {
                  "kind": "field",
                  "semantic_kind": "field",
                  "render_kind": "div",
                  "name": "NewTask",
                  "style": {
                    "Automatic": {
                      "style": "NewTask",
                      "source": {
                        "start": 166,
                        "end": 173
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
                              "start": 176,
                              "end": 181
                            }
                          }
                        },
                        "attributes": [
                          {
                            "name": "value",
                            "value": {
                              "Literal": "What needs to be done?"
                            },
                            "source": {
                              "start": 176,
                              "end": 214
                            }
                          }
                        ],
                        "bindings": [],
                        "events": [],
                        "conditions": [],
                        "children": [],
                        "source": {
                          "start": 176,
                          "end": 214
                        }
                      }
                    },
                    {
                      "Element": {
                        "kind": "input",
                        "semantic_kind": "input",
                        "render_kind": "input",
                        "name": "TaskInput",
                        "style": {
                          "Automatic": {
                            "style": "TaskInput",
                            "source": {
                              "start": 230,
                              "end": 239
                            }
                          }
                        },
                        "attributes": [
                          {
                            "name": "placeholder",
                            "value": {
                              "Literal": "Buy groceries..."
                            },
                            "source": {
                              "start": 270,
                              "end": 310
                            }
                          }
                        ],
                        "bindings": [
                          {
                            "property": "value",
                            "state": "draft",
                            "source": {
                              "start": 242,
                              "end": 269
                            }
                          }
                        ],
                        "events": [
                          {
                            "event": "keydown",
                            "modifiers": [
                              "enter"
                            ],
                            "handler": "addTask",
                            "source": {
                              "start": 311,
                              "end": 346
                            }
                          }
                        ],
                        "conditions": [],
                        "children": [],
                        "source": {
                          "start": 216,
                          "end": 356
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 154,
                    "end": 364
                  }
                }
              },
              {
                "Element": {
                  "kind": "action",
                  "semantic_kind": "action",
                  "render_kind": "button",
                  "name": "AddTask",
                  "style": {
                    "Explicit": {
                      "style": "PrimaryButton",
                      "source": {
                        "start": 387,
                        "end": 400
                      }
                    }
                  },
                  "attributes": [],
                  "bindings": [],
                  "events": [
                    {
                      "event": "press",
                      "modifiers": [],
                      "handler": "addTask",
                      "source": {
                        "start": 427,
                        "end": 452
                      }
                    }
                  ],
                  "conditions": [],
                  "children": [
                    {
                      "Text": {
                        "value": {
                          "Literal": "Add task"
                        },
                        "source": {
                          "start": 403,
                          "end": 426
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 366,
                    "end": 460
                  }
                }
              },
              {
                "Element": {
                  "kind": "list",
                  "semantic_kind": "list",
                  "render_kind": "ul",
                  "name": "TaskList",
                  "style": {
                    "Automatic": {
                      "style": "TaskList",
                      "source": {
                        "start": 473,
                        "end": 481
                      }
                    }
                  },
                  "attributes": [
                    {
                      "name": "source",
                      "value": {
                        "DataRef": "items"
                      },
                      "source": {
                        "start": 484,
                        "end": 505
                      }
                    }
                  ],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [
                    {
                      "List": {
                        "item": "item",
                        "collection": "items",
                        "key": "item.id",
                        "children": [
                          {
                            "Element": {
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "TaskRow",
                              "style": {
                                "Automatic": {
                                  "style": "TaskRow",
                                  "source": {
                                    "start": 563,
                                    "end": 570
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
                                    "name": "Complete",
                                    "style": {
                                      "Automatic": {
                                        "style": "Complete",
                                        "source": {
                                          "start": 592,
                                          "end": 600
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [
                                      {
                                        "event": "press",
                                        "modifiers": [],
                                        "handler": "toggleTask",
                                        "source": {
                                          "start": 629,
                                          "end": 663
                                        }
                                      }
                                    ],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Text": {
                                          "value": {
                                            "Literal": "Done"
                                          },
                                          "source": {
                                            "start": 603,
                                            "end": 628
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 573,
                                      "end": 677
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "DataRef": "item.label"
                                    },
                                    "source": {
                                      "start": 679,
                                      "end": 707
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 549,
                                "end": 719
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 507,
                          "end": 729
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 462,
                    "end": 737
                  }
                }
              }
            ],
            "source": {
              "start": 106,
              "end": 743
            }
          }
        }
      ],
      "capabilities": [
        "EventBinding",
        "TwoWayBinding",
        "ListRendering"
      ],
      "source": {
        "start": 0,
        "end": 749
      }
    }
  ]
} as const);

export default ir;
