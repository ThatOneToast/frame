// Generated typed Frame IR. Do not edit; regenerate with `frame build`.
// Source: src/app.frame
// Ownership: generated-only

import { defineFrameIrDocument } from '@frame/runtime-dom';

const ir = defineFrameIrDocument({
  "version": 1,
  "components": [
    {
      "name": "LLMDashboard",
      "props": [],
      "state": [],
      "slots": [],
      "nodes": [
        {
          "Element": {
            "kind": "screen",
            "semantic_kind": "screen",
            "render_kind": "div",
            "name": "AppShell",
            "style": {
              "Automatic": {
                "style": "AppShell",
                "source": {
                  "start": 232,
                  "end": 240
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
                  "name": "Header",
                  "style": {
                    "Automatic": {
                      "style": "Header",
                      "source": {
                        "start": 255,
                        "end": 261
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
                        "kind": "row",
                        "semantic_kind": "row",
                        "render_kind": "div",
                        "name": "NavBar",
                        "style": {
                          "Automatic": {
                            "style": "NavBar",
                            "source": {
                              "start": 276,
                              "end": 282
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
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "Logo",
                              "style": {
                                "Automatic": {
                                  "style": "Logo",
                                  "source": {
                                    "start": 301,
                                    "end": 305
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "LLM"
                                    },
                                    "source": {
                                      "start": 308,
                                      "end": 330
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Ops"
                                    },
                                    "source": {
                                      "start": 331,
                                      "end": 353
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 285,
                                "end": 365
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "SearchBar",
                              "style": {
                                "Automatic": {
                                  "style": "SearchBar",
                                  "source": {
                                    "start": 380,
                                    "end": 389
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Search models, runs, or metrics..."
                                    },
                                    "source": {
                                      "start": 392,
                                      "end": 445
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 366,
                                "end": 457
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "StatusCluster",
                              "style": {
                                "Automatic": {
                                  "style": "StatusCluster",
                                  "source": {
                                    "start": 472,
                                    "end": 485
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
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "StatusBadge",
                                    "style": {
                                      "Automatic": {
                                        "style": "StatusBadge",
                                        "source": {
                                          "start": 505,
                                          "end": 516
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Text": {
                                          "value": {
                                            "Literal": "4 models online"
                                          },
                                          "source": {
                                            "start": 519,
                                            "end": 555
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 488,
                                      "end": 569
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "row",
                                    "semantic_kind": "row",
                                    "render_kind": "div",
                                    "name": "UserInfo",
                                    "style": {
                                      "Automatic": {
                                        "style": "UserInfo",
                                        "source": {
                                          "start": 586,
                                          "end": 594
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Text": {
                                          "value": {
                                            "Literal": "admin@local"
                                          },
                                          "source": {
                                            "start": 597,
                                            "end": 629
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 570,
                                      "end": 643
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 458,
                                "end": 655
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 264,
                          "end": 665
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 243,
                    "end": 673
                  }
                }
              },
              {
                "Element": {
                  "kind": "panel",
                  "semantic_kind": "panel",
                  "render_kind": "div",
                  "name": "SidebarNav",
                  "style": {
                    "Automatic": {
                      "style": "SidebarNav",
                      "source": {
                        "start": 686,
                        "end": 696
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
                        "kind": "stack",
                        "semantic_kind": "stack",
                        "render_kind": "div",
                        "name": "Navigation",
                        "style": {
                          "Automatic": {
                            "style": "Navigation",
                            "source": {
                              "start": 713,
                              "end": 723
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
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavSection",
                              "style": {
                                "Automatic": {
                                  "style": "NavSection",
                                  "source": {
                                    "start": 742,
                                    "end": 752
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "NAVIGATE"
                                    },
                                    "source": {
                                      "start": 755,
                                      "end": 782
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 726,
                                "end": 794
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavGroupDash",
                              "style": {
                                "Automatic": {
                                  "style": "NavGroupDash",
                                  "source": {
                                    "start": 811,
                                    "end": 823
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Dashboard"
                                    },
                                    "source": {
                                      "start": 826,
                                      "end": 854
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Models"
                                    },
                                    "source": {
                                      "start": 855,
                                      "end": 880
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Inference"
                                    },
                                    "source": {
                                      "start": 881,
                                      "end": 909
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Agents"
                                    },
                                    "source": {
                                      "start": 910,
                                      "end": 935
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 795,
                                "end": 947
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavSection",
                              "style": {
                                "Automatic": {
                                  "style": "NavSection",
                                  "source": {
                                    "start": 964,
                                    "end": 974
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "ANALYTICS"
                                    },
                                    "source": {
                                      "start": 977,
                                      "end": 1005
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 948,
                                "end": 1017
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavGroupAnalytics",
                              "style": {
                                "Automatic": {
                                  "style": "NavGroupAnalytics",
                                  "source": {
                                    "start": 1034,
                                    "end": 1051
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Benchmarks"
                                    },
                                    "source": {
                                      "start": 1054,
                                      "end": 1083
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Context"
                                    },
                                    "source": {
                                      "start": 1084,
                                      "end": 1110
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Prompts"
                                    },
                                    "source": {
                                      "start": 1111,
                                      "end": 1137
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1018,
                                "end": 1149
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavSection",
                              "style": {
                                "Automatic": {
                                  "style": "NavSection",
                                  "source": {
                                    "start": 1166,
                                    "end": 1176
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "SYSTEM"
                                    },
                                    "source": {
                                      "start": 1179,
                                      "end": 1204
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1150,
                                "end": 1216
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavGroupSystem",
                              "style": {
                                "Automatic": {
                                  "style": "NavGroupSystem",
                                  "source": {
                                    "start": 1233,
                                    "end": 1247
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Logs"
                                    },
                                    "source": {
                                      "start": 1250,
                                      "end": 1273
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Settings"
                                    },
                                    "source": {
                                      "start": 1274,
                                      "end": 1301
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1217,
                                "end": 1313
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 699,
                          "end": 1323
                        }
                      }
                    },
                    {
                      "Element": {
                        "kind": "stack",
                        "semantic_kind": "stack",
                        "render_kind": "div",
                        "name": "SidebarFooter",
                        "style": {
                          "Automatic": {
                            "style": "SidebarFooter",
                            "source": {
                              "start": 1338,
                              "end": 1351
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
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "UserStatus",
                              "style": {
                                "Automatic": {
                                  "style": "UserStatus",
                                  "source": {
                                    "start": 1368,
                                    "end": 1378
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "admin@local"
                                    },
                                    "source": {
                                      "start": 1381,
                                      "end": 1411
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1354,
                                "end": 1423
                              }
                            }
                          },
                          {
                            "Text": {
                              "value": {
                                "Literal": "v0.4.2"
                              },
                              "source": {
                                "start": 1424,
                                "end": 1447
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 1324,
                          "end": 1457
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 674,
                    "end": 1465
                  }
                }
              },
              {
                "Element": {
                  "kind": "panel",
                  "semantic_kind": "panel",
                  "render_kind": "div",
                  "name": "MainContent",
                  "style": {
                    "Automatic": {
                      "style": "MainContent",
                      "source": {
                        "start": 1478,
                        "end": 1489
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
                        "kind": "stack",
                        "semantic_kind": "stack",
                        "render_kind": "div",
                        "name": "DashboardContent",
                        "style": {
                          "Automatic": {
                            "style": "DashboardContent",
                            "source": {
                              "start": 1506,
                              "end": 1522
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
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "MetricRow",
                              "style": {
                                "Automatic": {
                                  "style": "MetricRow",
                                  "source": {
                                    "start": 1539,
                                    "end": 1548
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
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "MetricCardModel",
                                    "style": {
                                      "Automatic": {
                                        "style": "MetricCardModel",
                                        "source": {
                                          "start": 1568,
                                          "end": 1583
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
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricIconModel",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricIconModel",
                                              "source": {
                                                "start": 1606,
                                                "end": 1621
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "MODEL"
                                                },
                                                "source": {
                                                  "start": 1624,
                                                  "end": 1652
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 1586,
                                            "end": 1668
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricContentModel",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricContentModel",
                                              "source": {
                                                "start": 1689,
                                                "end": 1707
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Active Model"
                                                },
                                                "source": {
                                                  "start": 1710,
                                                  "end": 1745
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Qwen3.6 Coder 27B"
                                                },
                                                "source": {
                                                  "start": 1746,
                                                  "end": 1786
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "MLX · 27B params"
                                                },
                                                "source": {
                                                  "start": 1787,
                                                  "end": 1831
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 1669,
                                            "end": 1847
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "card",
                                          "semantic_kind": "card",
                                          "render_kind": "div",
                                          "name": "DeltaModel",
                                          "style": {
                                            "Automatic": {
                                              "style": "DeltaModel",
                                              "source": {
                                                "start": 1867,
                                                "end": 1877
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Healthy"
                                                },
                                                "source": {
                                                  "start": 1880,
                                                  "end": 1910
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 1848,
                                            "end": 1926
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 1551,
                                      "end": 1940
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "MetricCardTps",
                                    "style": {
                                      "Automatic": {
                                        "style": "MetricCardTps",
                                        "source": {
                                          "start": 1958,
                                          "end": 1971
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
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricIconTps",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricIconTps",
                                              "source": {
                                                "start": 1994,
                                                "end": 2007
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "TPS"
                                                },
                                                "source": {
                                                  "start": 2010,
                                                  "end": 2036
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 1974,
                                            "end": 2052
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricContentTps",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricContentTps",
                                              "source": {
                                                "start": 2073,
                                                "end": 2089
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Tokens/sec"
                                                },
                                                "source": {
                                                  "start": 2092,
                                                  "end": 2125
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "42.5"
                                                },
                                                "source": {
                                                  "start": 2126,
                                                  "end": 2153
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "peak 51.2"
                                                },
                                                "source": {
                                                  "start": 2154,
                                                  "end": 2186
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2053,
                                            "end": 2202
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "card",
                                          "semantic_kind": "card",
                                          "render_kind": "div",
                                          "name": "DeltaTps",
                                          "style": {
                                            "Automatic": {
                                              "style": "DeltaTps",
                                              "source": {
                                                "start": 2222,
                                                "end": 2230
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "+8.3%"
                                                },
                                                "source": {
                                                  "start": 2233,
                                                  "end": 2261
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2203,
                                            "end": 2277
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 1941,
                                      "end": 2291
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "MetricCardCtx",
                                    "style": {
                                      "Automatic": {
                                        "style": "MetricCardCtx",
                                        "source": {
                                          "start": 2309,
                                          "end": 2322
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
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricIconCtx",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricIconCtx",
                                              "source": {
                                                "start": 2345,
                                                "end": 2358
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "CTX"
                                                },
                                                "source": {
                                                  "start": 2361,
                                                  "end": 2387
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2325,
                                            "end": 2403
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricContentCtx",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricContentCtx",
                                              "source": {
                                                "start": 2424,
                                                "end": 2440
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Context Window"
                                                },
                                                "source": {
                                                  "start": 2443,
                                                  "end": 2480
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "18,432 / 32,768"
                                                },
                                                "source": {
                                                  "start": 2481,
                                                  "end": 2519
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "56.3% utilized"
                                                },
                                                "source": {
                                                  "start": 2520,
                                                  "end": 2557
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2404,
                                            "end": 2573
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "card",
                                          "semantic_kind": "card",
                                          "render_kind": "div",
                                          "name": "DeltaCtx",
                                          "style": {
                                            "Automatic": {
                                              "style": "DeltaCtx",
                                              "source": {
                                                "start": 2593,
                                                "end": 2601
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "normal"
                                                },
                                                "source": {
                                                  "start": 2604,
                                                  "end": 2633
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2574,
                                            "end": 2649
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 2292,
                                      "end": 2663
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "MetricCardVram",
                                    "style": {
                                      "Automatic": {
                                        "style": "MetricCardVram",
                                        "source": {
                                          "start": 2681,
                                          "end": 2695
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
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricIconVram",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricIconVram",
                                              "source": {
                                                "start": 2718,
                                                "end": 2732
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "VRAM"
                                                },
                                                "source": {
                                                  "start": 2735,
                                                  "end": 2762
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2698,
                                            "end": 2778
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricContentVram",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricContentVram",
                                              "source": {
                                                "start": 2799,
                                                "end": 2816
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "GPU Memory"
                                                },
                                                "source": {
                                                  "start": 2819,
                                                  "end": 2852
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "14.2 GB / 24.0 GB"
                                                },
                                                "source": {
                                                  "start": 2853,
                                                  "end": 2893
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "NVIDIA RTX 4090"
                                                },
                                                "source": {
                                                  "start": 2894,
                                                  "end": 2932
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2779,
                                            "end": 2948
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "card",
                                          "semantic_kind": "card",
                                          "render_kind": "div",
                                          "name": "DeltaVram",
                                          "style": {
                                            "Automatic": {
                                              "style": "DeltaVram",
                                              "source": {
                                                "start": 2968,
                                                "end": 2977
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "59.2%"
                                                },
                                                "source": {
                                                  "start": 2980,
                                                  "end": 3008
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2949,
                                            "end": 3024
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 2664,
                                      "end": 3038
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1525,
                                "end": 3050
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "grid",
                              "semantic_kind": "grid",
                              "render_kind": "div",
                              "name": "PerformanceGrid",
                              "style": {
                                "Automatic": {
                                  "style": "PerformanceGrid",
                                  "source": {
                                    "start": 3066,
                                    "end": 3081
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
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "PerformanceChart",
                                    "style": {
                                      "Automatic": {
                                        "style": "PerformanceChart",
                                        "source": {
                                          "start": 3101,
                                          "end": 3117
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
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "ChartHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "ChartHeader",
                                              "source": {
                                                "start": 3140,
                                                "end": 3151
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
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ChartTitle",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartTitle",
                                                    "source": {
                                                      "start": 3174,
                                                      "end": 3184
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Inference Performance"
                                                      },
                                                      "source": {
                                                        "start": 3187,
                                                        "end": 3233
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "ChartRange",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ChartRange",
                                                          "source": {
                                                            "start": 3257,
                                                            "end": 3267
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "60m"
                                                            },
                                                            "source": {
                                                              "start": 3270,
                                                              "end": 3300
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 3234,
                                                        "end": 3320
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 3154,
                                                  "end": 3338
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ChartLegend",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartLegend",
                                                    "source": {
                                                      "start": 3359,
                                                      "end": 3370
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
                                                      "kind": "row",
                                                      "semantic_kind": "row",
                                                      "render_kind": "div",
                                                      "name": "LegendItemA",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "LegendItemA",
                                                          "source": {
                                                            "start": 3395,
                                                            "end": 3406
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Qwen3.6 Coder 27B"
                                                            },
                                                            "source": {
                                                              "start": 3409,
                                                              "end": 3453
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 3373,
                                                        "end": 3473
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "row",
                                                      "semantic_kind": "row",
                                                      "render_kind": "div",
                                                      "name": "LegendItemB",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "LegendItemB",
                                                          "source": {
                                                            "start": 3496,
                                                            "end": 3507
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Qwen3.5 0.8B Draft"
                                                            },
                                                            "source": {
                                                              "start": 3510,
                                                              "end": 3555
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 3474,
                                                        "end": 3575
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 3339,
                                                  "end": 3593
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 3120,
                                            "end": 3609
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "ChartArea",
                                          "style": {
                                            "Automatic": {
                                              "style": "ChartArea",
                                              "source": {
                                                "start": 3630,
                                                "end": 3639
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
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ChartAxis",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartAxis",
                                                    "source": {
                                                      "start": 3662,
                                                      "end": 3671
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:00"
                                                      },
                                                      "source": {
                                                        "start": 3674,
                                                        "end": 3704
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:15"
                                                      },
                                                      "source": {
                                                        "start": 3705,
                                                        "end": 3735
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:30"
                                                      },
                                                      "source": {
                                                        "start": 3736,
                                                        "end": 3766
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:45"
                                                      },
                                                      "source": {
                                                        "start": 3767,
                                                        "end": 3797
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:55"
                                                      },
                                                      "source": {
                                                        "start": 3798,
                                                        "end": 3828
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 3642,
                                                  "end": 3846
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 3610,
                                            "end": 3862
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 3084,
                                      "end": 3876
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "RunPrompt",
                                    "style": {
                                      "Automatic": {
                                        "style": "RunPrompt",
                                        "source": {
                                          "start": 3894,
                                          "end": 3903
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
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "PromptHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "PromptHeader",
                                              "source": {
                                                "start": 3926,
                                                "end": 3938
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Quick Inference Test"
                                                },
                                                "source": {
                                                  "start": 3941,
                                                  "end": 3984
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 3906,
                                            "end": 4000
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "PromptContent",
                                          "style": {
                                            "Automatic": {
                                              "style": "PromptContent",
                                              "source": {
                                                "start": 4021,
                                                "end": 4034
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
                                                "kind": "stack",
                                                "semantic_kind": "stack",
                                                "render_kind": "div",
                                                "name": "PromptInput",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "PromptInput",
                                                    "source": {
                                                      "start": 4059,
                                                      "end": 4070
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Enter a test prompt..."
                                                      },
                                                      "source": {
                                                        "start": 4073,
                                                        "end": 4120
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 4037,
                                                  "end": 4138
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "action",
                                                "semantic_kind": "action",
                                                "render_kind": "button",
                                                "name": "RunTest",
                                                "style": {
                                                  "Explicit": {
                                                    "style": "PrimaryButton",
                                                    "source": {
                                                      "start": 4170,
                                                      "end": 4183
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [
                                                  {
                                                    "event": "press",
                                                    "modifiers": [],
                                                    "handler": "runTest",
                                                    "source": {
                                                      "start": 4225,
                                                      "end": 4260
                                                    }
                                                  }
                                                ],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Run Inference"
                                                      },
                                                      "source": {
                                                        "start": 4186,
                                                        "end": 4224
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 4139,
                                                  "end": 4278
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 4001,
                                            "end": 4294
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "PromptMeta",
                                          "style": {
                                            "Automatic": {
                                              "style": "PromptMeta",
                                              "source": {
                                                "start": 4315,
                                                "end": 4325
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Model: Qwen3.6 Coder 27B"
                                                },
                                                "source": {
                                                  "start": 4328,
                                                  "end": 4375
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Provider: Local MLX"
                                                },
                                                "source": {
                                                  "start": 4376,
                                                  "end": 4418
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 4295,
                                            "end": 4434
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 3877,
                                      "end": 4448
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 3051,
                                "end": 4460
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "grid",
                              "semantic_kind": "grid",
                              "render_kind": "div",
                              "name": "DashboardGrid",
                              "style": {
                                "Automatic": {
                                  "style": "DashboardGrid",
                                  "source": {
                                    "start": 4476,
                                    "end": 4489
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
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "InferenceRuns",
                                    "style": {
                                      "Automatic": {
                                        "style": "InferenceRuns",
                                        "source": {
                                          "start": 4509,
                                          "end": 4522
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
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "RunsHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "RunsHeader",
                                              "source": {
                                                "start": 4545,
                                                "end": 4555
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
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunsTitleRow",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunsTitleRow",
                                                    "source": {
                                                      "start": 4578,
                                                      "end": 4590
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Recent Inference Runs"
                                                      },
                                                      "source": {
                                                        "start": 4593,
                                                        "end": 4639
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 4558,
                                                  "end": 4657
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunTabs",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunTabs",
                                                    "source": {
                                                      "start": 4678,
                                                      "end": 4685
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
                                                      "name": "TabAll",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "TabAll",
                                                          "source": {
                                                            "start": 4713,
                                                            "end": 4719
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [
                                                        {
                                                          "event": "press",
                                                          "modifiers": [],
                                                          "handler": "filterRuns",
                                                          "source": {
                                                            "start": 4753,
                                                            "end": 4793
                                                          }
                                                        }
                                                      ],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "All"
                                                            },
                                                            "source": {
                                                              "start": 4722,
                                                              "end": 4752
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 4688,
                                                        "end": 4813
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "action",
                                                      "semantic_kind": "action",
                                                      "render_kind": "button",
                                                      "name": "TabLocal",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "TabLocal",
                                                          "source": {
                                                            "start": 4839,
                                                            "end": 4847
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [
                                                        {
                                                          "event": "press",
                                                          "modifiers": [],
                                                          "handler": "filterRuns",
                                                          "source": {
                                                            "start": 4883,
                                                            "end": 4923
                                                          }
                                                        }
                                                      ],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Local"
                                                            },
                                                            "source": {
                                                              "start": 4850,
                                                              "end": 4882
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 4814,
                                                        "end": 4943
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "action",
                                                      "semantic_kind": "action",
                                                      "render_kind": "button",
                                                      "name": "TabRemote",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "TabRemote",
                                                          "source": {
                                                            "start": 4969,
                                                            "end": 4978
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [
                                                        {
                                                          "event": "press",
                                                          "modifiers": [],
                                                          "handler": "filterRuns",
                                                          "source": {
                                                            "start": 5015,
                                                            "end": 5055
                                                          }
                                                        }
                                                      ],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Remote"
                                                            },
                                                            "source": {
                                                              "start": 4981,
                                                              "end": 5014
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 4944,
                                                        "end": 5075
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 4658,
                                                  "end": 5093
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 4525,
                                            "end": 5109
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "RunsTableHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "RunsTableHeader",
                                              "source": {
                                                "start": 5130,
                                                "end": 5145
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
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunColHeaders",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunColHeaders",
                                                    "source": {
                                                      "start": 5168,
                                                      "end": 5181
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Model"
                                                      },
                                                      "source": {
                                                        "start": 5184,
                                                        "end": 5214
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Provider"
                                                      },
                                                      "source": {
                                                        "start": 5215,
                                                        "end": 5248
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Speed"
                                                      },
                                                      "source": {
                                                        "start": 5249,
                                                        "end": 5279
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Latency"
                                                      },
                                                      "source": {
                                                        "start": 5280,
                                                        "end": 5312
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Tokens"
                                                      },
                                                      "source": {
                                                        "start": 5313,
                                                        "end": 5344
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Cost"
                                                      },
                                                      "source": {
                                                        "start": 5345,
                                                        "end": 5374
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 5148,
                                                  "end": 5392
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 5110,
                                            "end": 5408
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "RunsTable",
                                          "style": {
                                            "Automatic": {
                                              "style": "RunsTable",
                                              "source": {
                                                "start": 5429,
                                                "end": 5438
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
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow1",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow1",
                                                    "source": {
                                                      "start": 5461,
                                                      "end": 5468
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
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel1",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel1",
                                                          "source": {
                                                            "start": 5495,
                                                            "end": 5504
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Qwen3.6 Coder 27B"
                                                            },
                                                            "source": {
                                                              "start": 5507,
                                                              "end": 5551
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "MLX local"
                                                            },
                                                            "source": {
                                                              "start": 5552,
                                                              "end": 5588
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 5471,
                                                        "end": 5608
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Local"
                                                      },
                                                      "source": {
                                                        "start": 5609,
                                                        "end": 5639
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "42.5 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 5640,
                                                        "end": 5675
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "23.5ms"
                                                      },
                                                      "source": {
                                                        "start": 5676,
                                                        "end": 5707
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "18,432"
                                                      },
                                                      "source": {
                                                        "start": 5708,
                                                        "end": 5739
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0000"
                                                      },
                                                      "source": {
                                                        "start": 5740,
                                                        "end": 5772
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 5441,
                                                  "end": 5790
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow2",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow2",
                                                    "source": {
                                                      "start": 5811,
                                                      "end": 5818
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
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel2",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel2",
                                                          "source": {
                                                            "start": 5845,
                                                            "end": 5854
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Gemma 3 27B"
                                                            },
                                                            "source": {
                                                              "start": 5857,
                                                              "end": 5895
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Ollama local"
                                                            },
                                                            "source": {
                                                              "start": 5896,
                                                              "end": 5935
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 5821,
                                                        "end": 5955
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Local"
                                                      },
                                                      "source": {
                                                        "start": 5956,
                                                        "end": 5986
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "35.8 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 5987,
                                                        "end": 6022
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "27.9ms"
                                                      },
                                                      "source": {
                                                        "start": 6023,
                                                        "end": 6054
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "12,288"
                                                      },
                                                      "source": {
                                                        "start": 6055,
                                                        "end": 6086
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0000"
                                                      },
                                                      "source": {
                                                        "start": 6087,
                                                        "end": 6119
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 5791,
                                                  "end": 6137
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow3",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow3",
                                                    "source": {
                                                      "start": 6158,
                                                      "end": 6165
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
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel3",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel3",
                                                          "source": {
                                                            "start": 6192,
                                                            "end": 6201
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Llama 3.1 8B"
                                                            },
                                                            "source": {
                                                              "start": 6204,
                                                              "end": 6243
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Together API"
                                                            },
                                                            "source": {
                                                              "start": 6244,
                                                              "end": 6283
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 6168,
                                                        "end": 6303
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Remote"
                                                      },
                                                      "source": {
                                                        "start": 6304,
                                                        "end": 6335
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "28.4 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 6336,
                                                        "end": 6371
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "35.2ms"
                                                      },
                                                      "source": {
                                                        "start": 6372,
                                                        "end": 6403
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "45,056"
                                                      },
                                                      "source": {
                                                        "start": 6404,
                                                        "end": 6435
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0042"
                                                      },
                                                      "source": {
                                                        "start": 6436,
                                                        "end": 6468
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 6138,
                                                  "end": 6486
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow4",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow4",
                                                    "source": {
                                                      "start": 6507,
                                                      "end": 6514
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
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel4",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel4",
                                                          "source": {
                                                            "start": 6541,
                                                            "end": 6550
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "DeepSeek Coder 6.7B"
                                                            },
                                                            "source": {
                                                              "start": 6553,
                                                              "end": 6599
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Local GPU"
                                                            },
                                                            "source": {
                                                              "start": 6600,
                                                              "end": 6636
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 6517,
                                                        "end": 6656
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Local"
                                                      },
                                                      "source": {
                                                        "start": 6657,
                                                        "end": 6687
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "31.2 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 6688,
                                                        "end": 6723
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "19.8ms"
                                                      },
                                                      "source": {
                                                        "start": 6724,
                                                        "end": 6755
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "8,192"
                                                      },
                                                      "source": {
                                                        "start": 6756,
                                                        "end": 6786
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0000"
                                                      },
                                                      "source": {
                                                        "start": 6787,
                                                        "end": 6819
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 6487,
                                                  "end": 6837
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow5",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow5",
                                                    "source": {
                                                      "start": 6858,
                                                      "end": 6865
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
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel5",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel5",
                                                          "source": {
                                                            "start": 6892,
                                                            "end": 6901
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Phi 3.5 Mini"
                                                            },
                                                            "source": {
                                                              "start": 6904,
                                                              "end": 6943
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Azure endpoint"
                                                            },
                                                            "source": {
                                                              "start": 6944,
                                                              "end": 6985
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 6868,
                                                        "end": 7005
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Remote"
                                                      },
                                                      "source": {
                                                        "start": 7006,
                                                        "end": 7037
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "52.1 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 7038,
                                                        "end": 7073
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "12.4ms"
                                                      },
                                                      "source": {
                                                        "start": 7074,
                                                        "end": 7105
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "4,096"
                                                      },
                                                      "source": {
                                                        "start": 7106,
                                                        "end": 7136
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0001"
                                                      },
                                                      "source": {
                                                        "start": 7137,
                                                        "end": 7169
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 6838,
                                                  "end": 7187
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 5409,
                                            "end": 7203
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 4492,
                                      "end": 7217
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "TopModels",
                                    "style": {
                                      "Automatic": {
                                        "style": "TopModels",
                                        "source": {
                                          "start": 7235,
                                          "end": 7244
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
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "TopModelsHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "TopModelsHeader",
                                              "source": {
                                                "start": 7267,
                                                "end": 7282
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Top Models"
                                                },
                                                "source": {
                                                  "start": 7285,
                                                  "end": 7318
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Ranked by throughput"
                                                },
                                                "source": {
                                                  "start": 7319,
                                                  "end": 7362
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 7247,
                                            "end": 7378
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "ModelsList",
                                          "style": {
                                            "Automatic": {
                                              "style": "ModelsList",
                                              "source": {
                                                "start": 7399,
                                                "end": 7409
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
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ModelRow1",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ModelRow1",
                                                    "source": {
                                                      "start": 7432,
                                                      "end": 7441
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
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "RankBadge1",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RankBadge1",
                                                          "source": {
                                                            "start": 7467,
                                                            "end": 7477
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "1"
                                                            },
                                                            "source": {
                                                              "start": 7480,
                                                              "end": 7508
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 7444,
                                                        "end": 7528
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "ModelInfo1",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ModelInfo1",
                                                          "source": {
                                                            "start": 7553,
                                                            "end": 7563
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Qwen3.6 Coder 27B"
                                                            },
                                                            "source": {
                                                              "start": 7566,
                                                              "end": 7610
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "MLX · 27B"
                                                            },
                                                            "source": {
                                                              "start": 7611,
                                                              "end": 7652
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 7529,
                                                        "end": 7672
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "+11.2%"
                                                      },
                                                      "source": {
                                                        "start": 7673,
                                                        "end": 7704
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 7412,
                                                  "end": 7722
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ModelRow2",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ModelRow2",
                                                    "source": {
                                                      "start": 7743,
                                                      "end": 7752
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
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "RankBadge2",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RankBadge2",
                                                          "source": {
                                                            "start": 7778,
                                                            "end": 7788
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "2"
                                                            },
                                                            "source": {
                                                              "start": 7791,
                                                              "end": 7819
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 7755,
                                                        "end": 7839
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "ModelInfo2",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ModelInfo2",
                                                          "source": {
                                                            "start": 7864,
                                                            "end": 7874
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Gemma 3 27B"
                                                            },
                                                            "source": {
                                                              "start": 7877,
                                                              "end": 7915
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Ollama · 27B"
                                                            },
                                                            "source": {
                                                              "start": 7916,
                                                              "end": 7960
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 7840,
                                                        "end": 7980
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "-0.8%"
                                                      },
                                                      "source": {
                                                        "start": 7981,
                                                        "end": 8011
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 7723,
                                                  "end": 8029
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ModelRow3",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ModelRow3",
                                                    "source": {
                                                      "start": 8050,
                                                      "end": 8059
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
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "RankBadge3",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RankBadge3",
                                                          "source": {
                                                            "start": 8085,
                                                            "end": 8095
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "3"
                                                            },
                                                            "source": {
                                                              "start": 8098,
                                                              "end": 8126
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8062,
                                                        "end": 8146
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "ModelInfo3",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ModelInfo3",
                                                          "source": {
                                                            "start": 8171,
                                                            "end": 8181
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Llama 3.1 8B"
                                                            },
                                                            "source": {
                                                              "start": 8184,
                                                              "end": 8223
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Together · 8B"
                                                            },
                                                            "source": {
                                                              "start": 8224,
                                                              "end": 8269
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8147,
                                                        "end": 8289
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "+1.8%"
                                                      },
                                                      "source": {
                                                        "start": 8290,
                                                        "end": 8320
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 8030,
                                                  "end": 8338
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ModelRow4",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ModelRow4",
                                                    "source": {
                                                      "start": 8359,
                                                      "end": 8368
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
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "RankBadge4",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RankBadge4",
                                                          "source": {
                                                            "start": 8394,
                                                            "end": 8404
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "4"
                                                            },
                                                            "source": {
                                                              "start": 8407,
                                                              "end": 8435
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8371,
                                                        "end": 8455
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "ModelInfo4",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ModelInfo4",
                                                          "source": {
                                                            "start": 8480,
                                                            "end": 8490
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Phi 3.5 Mini"
                                                            },
                                                            "source": {
                                                              "start": 8493,
                                                              "end": 8532
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Azure · 3.8B"
                                                            },
                                                            "source": {
                                                              "start": 8533,
                                                              "end": 8577
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8456,
                                                        "end": 8597
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "+2.6%"
                                                      },
                                                      "source": {
                                                        "start": 8598,
                                                        "end": 8628
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 8339,
                                                  "end": 8646
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 7379,
                                            "end": 8662
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 7218,
                                      "end": 8676
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 4461,
                                "end": 8688
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 1492,
                          "end": 8698
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 1466,
                    "end": 8706
                  }
                }
              }
            ],
            "source": {
              "start": 221,
              "end": 8712
            }
          }
        }
      ],
      "capabilities": [
        "EventBinding"
      ],
      "source": {
        "start": 28,
        "end": 8718
      }
    }
  ]
} as const);

export default ir;
